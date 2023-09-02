use std::time::SystemTime;
use chrono::prelude::*;

/// format var should be in chrono::format::strftime format. see the module for all escape
/// sequences. If you want to default to MLA format (what every frontend user should prob see),
/// then just set the format var to "None"
pub fn format_unix_timestamp(timestamp: SystemTime, format: Option<&str>) -> String {
    let timestamp_to_i64: i64 = timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp_to_i64, 0).unwrap();
    let formatted = match format {
        Some(format_str) => datetime.format(format_str).to_string(),
        None => datetime.format("%e %B %Y").to_string()
    };
    formatted.trim().to_string()
}
