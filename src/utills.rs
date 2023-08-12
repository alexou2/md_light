use chrono::{Datelike, Local, Timelike};

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
        0,
        0,
        0
    );

    println!("{}", formatted_time);
    formatted_time
}
