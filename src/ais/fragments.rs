//! AIS multi-fragment message reassembly.
//!
//! AIS messages can span multiple NMEA sentences (up to 5 fragments).
//! The reassembler collects fragments by message ID (0-9) and returns
//! the complete payload when the last fragment arrives.

use super::AisDecodeError;

/// Assembled AIS payload ready for decoding.
#[derive(Debug, Clone)]
pub struct AisPayload {
    /// Combined 6-bit armored payload from all fragments.
    pub payload: String,
    /// Fill bits from the last fragment (0-5).
    pub fill_bits: u8,
    /// Channel indicator ('A' or 'B').
    pub channel: char,
}

/// Slot for tracking a partially assembled multi-fragment message.
#[derive(Debug, Default)]
struct FragmentSlot {
    /// Total expected fragments.
    total: u8,
    /// Number of fragments received so far.
    received: u8,
    /// Accumulated payload.
    payload: String,
}

/// Maximum accumulated payload size in characters.
///
/// ITU-R M.1371-5 allows at most 5 TDMA slots (128 + 4×256 = 1152 bits).
/// At 6 bits per armored character that's 192 characters. We use 256 to
/// allow headroom for non-conforming implementations.
const MAX_PAYLOAD_SIZE: usize = 256;

/// Maximum number of fragments per message.
///
/// The AIVDM/AIVDO sentence format uses a single-digit fragment count (1-9),
/// but ITU-R M.1371-5 limits transmissions to 5 TDMA slots. 5 fragments is
/// sufficient for the 1152-bit maximum.
const MAX_FRAGMENTS: u8 = 5;

/// Map a valid AIS channel field to a slot row (A/1 → 0, B/2 → 1).
fn channel_index(channel: &str) -> Option<usize> {
    match channel {
        "" | "A" | "1" => Some(0),
        "B" | "2" => Some(1),
        _ => None,
    }
}

/// Multi-fragment reassembler.
///
/// Maintains 10 slots (message IDs 0-9) per channel for concurrent
/// multi-fragment message assembly. Enforces payload size and fragment count
/// limits to prevent unbounded memory growth from malformed input.
pub struct FragmentCollector {
    // Indexed [channel_index][message_id] so concurrent A/B messages with the
    // same message id don't collide.
    slots: [[Option<FragmentSlot>; 10]; 2],
}

impl FragmentCollector {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
        }
    }

    /// Process one AIS NMEA frame's fields.
    ///
    /// Fields expected (from frame.fields after VDM/VDO sentence type):
    /// - `[0]` total_fragments
    /// - `[1]` fragment_number
    /// - `[2]` message_id (may be empty for single-fragment)
    /// - `[3]` channel ('A'/'B')
    /// - `[4]` payload (6-bit armored)
    /// - `[5]` fill_bits
    ///
    /// Returns `Some(AisPayload)` when a complete message is assembled.
    /// Returns `None` for pending fragments and invalid input. Use
    /// [`Self::process_checked`] to distinguish those cases.
    pub fn process(&mut self, fields: &[&str]) -> Option<AisPayload> {
        self.process_checked(fields).ok().flatten()
    }

    /// Process VDM/VDO fields with diagnostics for rejected fragments.
    ///
    /// Returns `Ok(None)` while awaiting more fragments, including a repeated
    /// continuation already accepted by the assembly. A sequence mismatch or an
    /// oversized continuation discards the affected assembly; other slots are
    /// retained. Field validation errors and oversized first fragments leave
    /// assemblies unchanged.
    pub fn process_checked(
        &mut self,
        fields: &[&str],
    ) -> Result<Option<AisPayload>, AisDecodeError> {
        if fields.len() < 6 {
            return Err(AisDecodeError::MissingFragmentFields {
                actual: fields.len(),
            });
        }

        let total: u8 = fields[0]
            .parse()
            .map_err(|_| AisDecodeError::InvalidFragmentField {
                field: "total_fragments",
            })?;
        let frag_num: u8 = fields[1]
            .parse()
            .map_err(|_| AisDecodeError::InvalidFragmentField {
                field: "fragment_number",
            })?;
        let msg_id_str = fields[2];
        let channel_field = fields[3];
        let channel_index = channel_index(channel_field)
            .ok_or(AisDecodeError::InvalidFragmentField { field: "channel" })?;
        let channel = if channel_index == 0 { 'A' } else { 'B' };
        let payload = fields[4];
        let fill_bits: u8 = fields[5]
            .parse()
            .ok()
            .filter(|&n| n <= 5)
            .ok_or(AisDecodeError::InvalidFragmentField { field: "fill_bits" })?;

        if total == 0 || total > MAX_FRAGMENTS {
            return Err(AisDecodeError::InvalidFragmentField {
                field: "total_fragments",
            });
        }
        if frag_num == 0 || frag_num > total {
            return Err(AisDecodeError::InvalidFragmentField {
                field: "fragment_number",
            });
        }

        // Single-fragment message — return immediately
        if total == 1 {
            if payload.len() > MAX_PAYLOAD_SIZE {
                return Err(AisDecodeError::PayloadTooLong {
                    actual: payload.len(),
                    maximum: MAX_PAYLOAD_SIZE,
                });
            }
            return Ok(Some(AisPayload {
                payload: payload.to_string(),
                fill_bits,
                channel,
            }));
        }

        // Multi-fragment — need a message ID
        let msg_id: usize =
            msg_id_str
                .parse()
                .map_err(|_| AisDecodeError::InvalidFragmentField {
                    field: "message_id",
                })?;
        if msg_id > 9 {
            return Err(AisDecodeError::InvalidFragmentField {
                field: "message_id",
            });
        }
        let ch = channel_index;

        if frag_num == 1 {
            // Start new assembly
            if payload.len() > MAX_PAYLOAD_SIZE {
                return Err(AisDecodeError::PayloadTooLong {
                    actual: payload.len(),
                    maximum: MAX_PAYLOAD_SIZE,
                });
            }
            self.slots[ch][msg_id] = Some(FragmentSlot {
                total,
                received: 1,
                payload: payload.to_string(),
            });
            Ok(None)
        } else {
            // Continue assembly
            let slot = self.slots[ch][msg_id]
                .as_mut()
                .ok_or(AisDecodeError::UnexpectedFragment)?;
            // A re-sent fragment (same number as the last received) is idempotent:
            // ignore it without discarding the in-progress assembly.
            if frag_num == slot.received {
                return Ok(None);
            }
            if slot.total != total || slot.received + 1 != frag_num {
                // Out of sequence — discard
                self.slots[ch][msg_id] = None;
                return Err(AisDecodeError::UnexpectedFragment);
            }

            let actual = slot.payload.len() + payload.len();
            if actual > MAX_PAYLOAD_SIZE {
                self.slots[ch][msg_id] = None;
                return Err(AisDecodeError::PayloadTooLong {
                    actual,
                    maximum: MAX_PAYLOAD_SIZE,
                });
            }
            slot.payload.push_str(payload);
            slot.received = frag_num;

            if frag_num == total {
                // Complete — take the slot
                let completed = self.slots[ch][msg_id]
                    .take()
                    .ok_or(AisDecodeError::UnexpectedFragment)?;
                Ok(Some(AisPayload {
                    payload: completed.payload,
                    fill_bits,
                    channel,
                }))
            } else {
                Ok(None)
            }
        }
    }
}

impl Default for FragmentCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_fragment_errors_and_pending_are_distinct() {
        use crate::ais::AisDecodeError;
        let mut c = FragmentCollector::new();
        assert_eq!(
            c.process_checked(&[]).err(),
            Some(AisDecodeError::MissingFragmentFields { actual: 0 })
        );
        assert_eq!(
            c.process_checked(&["1", "1", "", "Z", "1", "0"]).err(),
            Some(AisDecodeError::InvalidFragmentField { field: "channel" })
        );
        assert!(matches!(
            c.process_checked(&["2", "1", "0", "A", "1", "0"]),
            Ok(None)
        ));
        assert_eq!(
            c.process_checked(&["2", "2", "1", "A", "1", "0"]).err(),
            Some(AisDecodeError::UnexpectedFragment)
        );
        let complete = c
            .process_checked(&["2", "2", "0", "A", "2", "0"])
            .expect("valid continuation")
            .expect("complete");
        assert_eq!(complete.payload, "12");
    }

    #[test]
    fn checked_fragment_field_and_payload_limits() {
        use crate::ais::AisDecodeError;
        for (index, value, field) in [
            (0, "0", "total_fragments"),
            (0, "6", "total_fragments"),
            (0, "bad", "total_fragments"),
            (1, "0", "fragment_number"),
            (1, "3", "fragment_number"),
            (1, "bad", "fragment_number"),
            (2, "10", "message_id"),
            (2, "", "message_id"),
            (3, "Z", "channel"),
            (5, "6", "fill_bits"),
            (5, "bad", "fill_bits"),
        ] {
            let mut fields = ["2", "1", "0", "A", "1", "0"];
            fields[index] = value;
            assert_eq!(
                FragmentCollector::new().process_checked(&fields).err(),
                Some(AisDecodeError::InvalidFragmentField { field }),
                "{fields:?}"
            );
        }
        let oversized = "1".repeat(257);
        for total in ["1", "2"] {
            assert_eq!(
                FragmentCollector::new()
                    .process_checked(&[total, "1", "0", "A", &oversized, "0"])
                    .err(),
                Some(AisDecodeError::PayloadTooLong {
                    actual: 257,
                    maximum: 256
                })
            );
        }
        let mut c = FragmentCollector::new();
        let at_limit = "1".repeat(256);
        assert!(matches!(
            c.process_checked(&["2", "1", "0", "A", &at_limit, "0"]),
            Ok(None)
        ));
        assert_eq!(
            c.process_checked(&["2", "2", "0", "A", "1", "0"]).err(),
            Some(AisDecodeError::PayloadTooLong {
                actual: 257,
                maximum: 256
            })
        );
        assert_eq!(
            c.process_checked(&["2", "2", "0", "A", "", "0"]).err(),
            Some(AisDecodeError::UnexpectedFragment)
        );
    }

    #[test]
    fn checked_fragments_preserve_compatibility_and_other_slots() {
        use crate::ais::AisDecodeError;
        let mut c = FragmentCollector::new();
        for channel in ["", "1", "2", "A", "B"] {
            let payload = c
                .process_checked(&["1", "1", "ignored", channel, "1", "0", "extra"])
                .expect("compatible fields")
                .expect("complete");
            assert_eq!(
                payload.channel,
                if matches!(channel, "2" | "B") {
                    'B'
                } else {
                    'A'
                }
            );
        }
        for channel in ["A", "B"] {
            assert!(matches!(
                c.process_checked(&["3", "1", "0", channel, "1", "0"]),
                Ok(None)
            ));
        }
        assert!(matches!(
            c.process_checked(&["3", "2", "0", "A", "2", "0"]),
            Ok(None)
        ));
        assert!(matches!(
            c.process_checked(&["3", "2", "0", "A", "2", "0"]),
            Ok(None)
        ));
        assert_eq!(
            c.process_checked(&["3", "3", "0", "B", "3", "0"]).err(),
            Some(AisDecodeError::UnexpectedFragment)
        );
        let payload = c
            .process_checked(&["3", "3", "0", "A", "3", "0"])
            .expect("A unaffected")
            .expect("complete");
        assert_eq!(payload.payload, "123");
        assert_eq!(
            c.process_checked(&["3", "2", "0", "B", "2", "0"]).err(),
            Some(AisDecodeError::UnexpectedFragment)
        );
    }

    #[test]
    fn multi_fragment() {
        let mut c = FragmentCollector::new();

        // Fragment 1 of 2
        let r1 = c.process(&[
            "2",
            "1",
            "0",
            "A",
            "53brRt4000010SG700iE@LE8@Tp4000000000153P615t0Ht0SCkjH4jC1C",
            "0",
        ]);
        assert!(r1.is_none());

        // Fragment 2 of 2
        let r2 = c.process(&["2", "2", "0", "A", "`0000000001", "2"]);
        assert!(r2.is_some());
        let p = r2.expect("valid");
        assert!(p.payload.starts_with("53brRt"));
        assert!(p.payload.ends_with("`0000000001"));
        assert_eq!(p.fill_bits, 2);
    }

    #[test]
    fn out_of_sequence_discards() {
        let mut c = FragmentCollector::new();

        // Fragment 1 of 3
        let _ = c.process(&["3", "1", "1", "A", "AAAA", "0"]);
        // Fragment 3 of 3 (skipped 2) — should discard
        let r = c.process(&["3", "3", "1", "A", "CCCC", "0"]);
        assert!(r.is_none());
        // Slot should be cleared
        assert!(c.slots[0][1].is_none());
    }

    #[test]
    fn single_fragment() {
        let mut c = FragmentCollector::new();
        let result = c.process(&["1", "1", "", "A", "13u@Dt002s000000000000000000", "0"]);
        assert!(result.is_some());
        let p = result.expect("valid");
        assert_eq!(p.payload, "13u@Dt002s000000000000000000");
        assert_eq!(p.fill_bits, 0);
        assert_eq!(p.channel, 'A');
    }

    #[test]
    fn single_fragment_enforces_payload_limit() {
        let mut c = FragmentCollector::new();
        let oversized = "A".repeat(MAX_PAYLOAD_SIZE + 1);
        assert!(
            c.process(&["1", "1", "", "A", oversized.as_str(), "0"])
                .is_none(),
            "single-fragment payload over MAX_PAYLOAD_SIZE must be rejected"
        );
        let at_limit = "A".repeat(MAX_PAYLOAD_SIZE);
        assert!(
            c.process(&["1", "1", "", "A", at_limit.as_str(), "0"])
                .is_some()
        );
    }

    #[test]
    fn numeric_channel_normalizes_to_letter() {
        let mut c = FragmentCollector::new();
        let p = c
            .process(&["1", "1", "", "1", "13u@Dt002s000000000000000000", "0"])
            .expect("valid single fragment");
        assert_eq!(p.channel, 'A');
        let p = c
            .process(&["1", "1", "", "2", "13u@Dt002s000000000000000000", "0"])
            .expect("valid single fragment");
        assert_eq!(p.channel, 'B');
    }

    #[test]
    fn total_count_mismatch_discards() {
        let mut c = FragmentCollector::new();
        // Fragment 1 says total=2
        let _ = c.process(&["2", "1", "0", "A", "AAAA", "0"]);
        // Fragment 2 says total=3 — mismatch
        let r = c.process(&["3", "2", "0", "A", "BBBB", "0"]);
        assert!(r.is_none());
        assert!(c.slots[0][0].is_none());
    }

    #[test]
    fn too_few_fields() {
        let mut c = FragmentCollector::new();
        assert!(c.process(&["1", "1"]).is_none());
    }

    #[test]
    fn zero_total_rejected() {
        let mut c = FragmentCollector::new();
        assert!(c.process(&["0", "1", "", "A", "payload", "0"]).is_none());
    }

    #[test]
    fn invalid_fill_bits_rejected() {
        let mut c = FragmentCollector::new();
        // fill_bits 7 is out of range (valid 0-5) -> reject.
        assert!(
            c.process(&["1", "1", "", "A", "13u@Dt002s000000000000000000", "7"])
                .is_none()
        );
    }

    #[test]
    fn invalid_channel_cannot_start_assembly() {
        let mut c = FragmentCollector::new();
        assert!(c.process(&["2", "1", "0", "Z", "AAAA", "0"]).is_none());
        assert!(c.process(&["2", "2", "0", "A", "BBBB", "0"]).is_none());
        assert!(c.slots[0][0].is_none());
    }

    #[test]
    fn concurrent_channels_same_id() {
        let mut c = FragmentCollector::new();
        // Two 2-fragment messages, both message-id 0, on channels A and B, interleaved.
        assert!(c.process(&["2", "1", "0", "A", "53brRt", "0"]).is_none());
        assert!(c.process(&["2", "1", "0", "B", "AAAA", "0"]).is_none());
        let a = c
            .process(&["2", "2", "0", "A", "`0000000001", "2"])
            .expect("channel A completes");
        assert!(a.payload.starts_with("53brRt"));
        assert_eq!(a.channel, 'A');
        let b = c
            .process(&["2", "2", "0", "B", "BBBB", "0"])
            .expect("channel B completes");
        assert!(b.payload.starts_with("AAAA"));
        assert_eq!(b.channel, 'B');
    }

    #[test]
    fn duplicate_fragment_is_idempotent() {
        let mut c = FragmentCollector::new();
        assert!(c.process(&["3", "1", "1", "A", "AAAA", "0"]).is_none());
        assert!(c.process(&["3", "2", "1", "A", "BBBB", "0"]).is_none());
        // Re-sent fragment 2 must be ignored, NOT discard the whole assembly.
        assert!(c.process(&["3", "2", "1", "A", "BBBB", "0"]).is_none());
        let done = c
            .process(&["3", "3", "1", "A", "CCCC", "0"])
            .expect("completes despite the duplicate");
        assert_eq!(done.payload, "AAAABBBBCCCC");
    }

    #[test]
    fn fragment_channel_mismatch_discarded() {
        let mut c = FragmentCollector::new();
        // Fragment 1 of 2 on channel A (message id 0).
        assert!(c.process(&["2", "1", "0", "A", "AAAA", "0"]).is_none());
        // Fragment 2 of 2, SAME message id, but on channel B — no slot in row B.
        assert!(c.process(&["2", "2", "0", "B", "BBBB", "0"]).is_none());
        // Channel A's in-progress assembly is untouched; its own fragment 2 completes it.
        let done = c
            .process(&["2", "2", "0", "A", "CCCC", "2"])
            .expect("A completes");
        assert_eq!(done.payload, "AAAACCCC");
        assert_eq!(done.channel, 'A');
    }
}
