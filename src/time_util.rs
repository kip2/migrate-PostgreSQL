use chrono::{Local, Utc};

pub fn unix_time_stamp() -> String {
    let now = Utc::now();
    let unix_time_stamp = now.timestamp();
    unix_time_stamp.to_string()
}

pub fn jp_date() -> String {
    let current_date = Local::now();
    let formatted_date = current_date.format("%Y-%m-%d").to_string();
    formatted_date
}
