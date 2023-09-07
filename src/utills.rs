use actix_web::HttpRequest;
use chrono::{Datelike, Timelike};

// returns the local time, offset by a month
pub fn get_offset_time() -> String {
    let current_time = chrono::Local::now();

    let mut year = current_time.year();
    let mut month = current_time.month();
    let mut day = current_time.day();
    // set the month to devaber of previous year if the current month is january

    // adjusts the day for the end of the month to fix issues like getting mangas created on february 30th

    if month == 1 {
        year -= 1;
        month = 12;
    } else {
        month -= 1;
    }

    if day >= 28 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => todo!(),
            4 | 6 | 9 | 11 => todo!(),
            2 => todo!(),
            _ => unreachable!(),
        }
    }
    let offset_time = current_time
        .checked_sub_days(chrono::Days::new(30))
        .expect("error while offsetting time");
    // 2023-07-08T11 %3A 44 %3A 57
    // 2023-08-07T15 %3A 55 %3A 20
    let formatted_time = format!(
        "{:04}-{:02}-{:02}T{:02}%3A{:02}%3A{:02}",
        // year,
        // month,
        // day,
        // offset_time.year(),
        // offset_time.month(),
        year,
        month,
        offset_time.day()+1,
        // offset_time.hour(),
        // offset_time.minute(),
        // offset_time.second()
        0,0,0
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
fn offset_day_if_necessary(day:u32, month:u32) {}
