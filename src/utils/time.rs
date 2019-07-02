
use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_stamp_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms;
}

pub fn time_stamp_s() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_s = since_the_epoch.as_secs() ;
    return in_s;
}
