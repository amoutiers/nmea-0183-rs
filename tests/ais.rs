#[cfg(feature = "abm")]
#[path = "ais/abm.rs"]
mod abm;
#[cfg(feature = "bbm")]
#[path = "ais/bbm.rs"]
mod bbm;
#[cfg(feature = "ais")]
#[path = "ais/parser.rs"]
mod parser;
#[cfg(feature = "ais")]
#[path = "ais/transmit.rs"]
mod transmit;
#[cfg(feature = "ais")]
#[path = "ais/type1.rs"]
mod type1;
#[cfg(feature = "ais")]
#[path = "ais/type11.rs"]
mod type11;
#[cfg(feature = "ais")]
#[path = "ais/type12.rs"]
mod type12;
#[cfg(feature = "ais")]
#[path = "ais/type13.rs"]
mod type13;
#[cfg(feature = "ais")]
#[path = "ais/type14.rs"]
mod type14;
#[cfg(feature = "ais")]
#[path = "ais/type15.rs"]
mod type15;
#[cfg(feature = "ais")]
#[path = "ais/type18.rs"]
mod type18;
#[cfg(feature = "ais")]
#[path = "ais/type19.rs"]
mod type19;
#[cfg(feature = "ais")]
#[path = "ais/type21.rs"]
mod type21;
#[cfg(feature = "ais")]
#[path = "ais/type24.rs"]
mod type24;
#[cfg(feature = "ais")]
#[path = "ais/type27.rs"]
mod type27;
#[cfg(feature = "ais")]
#[path = "ais/type4.rs"]
mod type4;
#[cfg(feature = "ais")]
#[path = "ais/type5.rs"]
mod type5;
#[cfg(feature = "ais")]
#[path = "ais/type6.rs"]
mod type6;
#[cfg(feature = "ais")]
#[path = "ais/type7.rs"]
mod type7;
#[cfg(feature = "ais")]
#[path = "ais/type8.rs"]
mod type8;
#[cfg(feature = "ais")]
#[path = "ais/type9.rs"]
mod type9;

#[cfg(feature = "ais")]
use nmea_0183_rs::ais::{AisDecodeOutcome, AisMessage};

#[cfg(feature = "ais")]
fn expect_message(outcome: AisDecodeOutcome) -> AisMessage {
    match outcome {
        AisDecodeOutcome::Message(message) => message,
        other => panic!("expected complete AIS message, got {other:?}"),
    }
}
