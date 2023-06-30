use std::io;
use std::io::{Read, Write};

pub fn get_converted_number_from_string(any_string: String) -> i32 {
    return any_string.parse().unwrap();
}

pub fn get_user_input(mut any_variable: String, information_requested: String) -> String {
    print!("Please inform: {}: ", information_requested.trim().to_string());
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut any_variable).expect("Failed to read line");
    return any_variable.trim().to_string();
}
