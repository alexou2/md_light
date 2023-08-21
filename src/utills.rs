use actix_web::HttpRequest;
use chrono::{Datelike, Timelike};

// returns the local time, offset by a month
pub fn get_offset_time() -> String {
    let current_time = chrono::Local::now();

    let mut year = current_time.year();
    let mut month = current_time.month();
    if month - 1 < 1 {
        month = 12;
        year -= 1;
    } else {
        month -= 1
    }
    let formatted_time = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year,
        month,
        current_time.day() + 1,
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    );

    println!("{}", formatted_time);
    formatted_time
}
pub fn check_localhost(path: HttpRequest) -> bool {
    let is_localhost = path.connection_info().host() == "172.0.0.1:8080"
        || path.connection_info().host() == "localhost:8080";
    is_localhost
}
