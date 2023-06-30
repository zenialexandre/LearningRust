use std::io;
use std::io::Write;
use crate::helper;

pub fn init_leap_year() {
    let mut inputted_year = String::new();
    inputted_year = helper::get_user_input(inputted_year, "Year to be checked".to_string());

    if consist_leap_year(helper::get_converted_number_from_string(inputted_year.trim().to_string())) {
        println!("It's a leap year!");
    } else {
        println!("Isn't a leap year!");
    }
}

fn consist_leap_year(converted_inputted_year: i32) -> bool {
    return converted_inputted_year % 4 == 0;
}
