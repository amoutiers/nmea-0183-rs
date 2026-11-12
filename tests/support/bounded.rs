use std::process::Command;
use std::time::{Duration, Instant};

pub(crate) fn run_bounded(test_name: &str, body: impl FnOnce()) {
    const CHILD_ENV: &str = "NMEA_BOUNDED_TEST_CHILD";
    const COMPLETED: i32 = 42;
    if std::env::var(CHILD_ENV).ok().as_deref() == Some(test_name) {
        body();
        // A normal harness exit also means zero selected tests, so require acknowledgement.
        std::process::exit(COMPLETED);
    }

    let mut child = Command::new(std::env::current_exe().expect("test executable"))
        .args(["--exact", test_name, "--test-threads=1"])
        .env(CHILD_ENV, test_name)
        .spawn()
        .expect("start bounded test");
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                assert_eq!(
                    status.code(),
                    Some(COMPLETED),
                    "{test_name}: child failed or test was not selected: {status}"
                );
                return;
            }
            Ok(None) if start.elapsed() < Duration::from_secs(10) => {
                std::thread::sleep(Duration::from_millis(10));
            }
            outcome => {
                let _ = child.kill();
                let status = child.wait().expect("reap bounded test child");
                panic!("{test_name}: child did not finish within 10s: {outcome:?}, {status}");
            }
        }
    }
}
