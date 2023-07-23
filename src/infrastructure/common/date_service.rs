use std::time::{SystemTime, UNIX_EPOCH};

pub struct DateService;

impl DateService {
    pub fn get_cuurent_timestamp() -> u64 {
        let current_time = SystemTime::now();

        current_time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }
}
