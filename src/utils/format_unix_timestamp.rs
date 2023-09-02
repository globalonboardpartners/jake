use std::time::SystemTime;
use chrono::prelude::*;

/// format_str should be in chrono::format::strftime format. see the module for all escape
/// sequences
pub fn format_unix_timestamp(timestamp: SystemTime, format_str: &str) -> String {
    let timestamp_to_i64: i64 = timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp_to_i64, 0).unwrap();
    let formatted = datetime.format(format_str).to_string();
    formatted.trim().to_string()
}
