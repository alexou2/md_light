use actix_web::HttpRequest;
use chrono::{Datelike, Timelike};

/// returns the local time, offset by a month for the homepage feed
pub fn get_offset_time() -> String {
    let current_time = chrono::Local::now();

    // offsets the time to get the same feed as mangadex's popular titles
    let offset_time = current_time
        .checked_sub_days(chrono::Days::new(30))
        .expect("Time couldn't be correctly offset");

    // 2023-07-08T11 %3A 44 %3A 57
    // 2023-08-07T15 %3A 55 %3A 20
    let formatted_time = format!(
        "{:04}-{:02}-{:02}T{:02}%3A{:02}%3A{:02}",
        offset_time.year(),
        offset_time.month(),
        offset_time.day(),
        offset_time.hour(),
        offset_time.minute(),
        offset_time.second()
    );
    formatted_time
}

/// checks if the request comes from 172.0.0.1 (localhost)
pub fn check_localhost(path: &HttpRequest) -> bool {
    let binding = path.connection_info();
    let ip = binding.peer_addr().expect("unable to get client IP");
    
    let is_localhost = match ip {
        "172.0.0.1" | "localhost" => true,
        _ => false,
    };
    is_localhost
}

pub fn get_current_time() -> String {
    let current_time = chrono::Local::now();
    let formatted_time = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    );
    formatted_time
}
