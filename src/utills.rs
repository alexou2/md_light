use actix_web::HttpRequest;
use chrono::{Datelike, Timelike};

use crate::md_struct::Chapters;

// returns the local time, offset by a month
pub fn get_offset_time() -> String {
    let current_time = chrono::Local::now();

    let mut year = current_time.year();
    let mut month = current_time.month();
    let mut day = current_time.month() + 1;
    // set the month to devaber of previous year if the current month is january
    if month - 1 < 1 {
        month = 12;
        year -= 1;
    } else {
        month -= 1
    }
    // adjusts the day for the end of the month to fix issues like getting mangas created on february 30th
    let day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29 // Leap year
            } else {
                28 // Non-leap year
            }
        }
        _ => unreachable!(),
    };
    let formatted_time = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year,
        month,
        // current_time.day() + 1,
        day,
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    );
    println!("{}", formatted_time);
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

// sorting the chapters by number and places Oneshots at the start of the list
pub fn sort_by_chapter(mut chapter_list: Vec<Chapters>) -> Vec<Chapters> {
    let list_length = chapter_list.len();
    println!("{}", list_length);
    for j in 0..list_length {
        for i in 0..list_length - j - 1 {
            // println!("{}",i);
            if chapter_list[i].chapter_number > chapter_list[i + 1].chapter_number {
                let small_temp = chapter_list[i].clone();
                chapter_list[i] = chapter_list[i + 1].clone();
                chapter_list[i + 1] = small_temp;
            }
        }
    }

    chapter_list
}
