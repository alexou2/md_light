use actix_web::HttpRequest;
use chrono::{Datelike, Timelike};

// returns the local time, offset by a month
pub fn get_offset_time() -> String {
    let current_time = chrono::Local::now();

    // let mut year = current_time.year();
    // let mut month = current_time.month();
    // let mut day = current_time.day();
    // set the month to devaber of previous year if the current month is january

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
        offset_time.second() // 0,
                             // 0,
                             // 0
    );
    println!("date-1month = {}", formatted_time);
    formatted_time
}
// checks if the request comes from the localhost ip or another one
pub fn check_localhost(path: &HttpRequest) -> bool {
    let is_localhost = path
        .connection_info()
        .peer_addr()
        .expect("unable to get client ID")
        == "127.0.0.1"
        || path
            .connection_info()
            .peer_addr()
            .expect("unable to get client ID")
            == "localhost";
    println!("{}", is_localhost);
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
