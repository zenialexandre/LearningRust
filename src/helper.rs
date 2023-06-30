use std::io;
use std::io::Write;

pub fn get_converted_number_from_string(any_string: String) -> i32 {
    return any_string.parse().unwrap();
}

pub fn get_converted_number_from_char(any_char: char) -> i32 {
    return any_char.to_digit(10).unwrap() as i32;
}

pub fn get_converted_u32_from_usize(any_usize: usize) -> u32 {
    return any_usize as u32;
}

pub fn get_user_input(mut any_variable: String, information_requested: String) -> String {
    print!("Please inform: {}: ", information_requested.trim().to_string());
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut any_variable).expect("Failed to read line");
    return any_variable.trim().to_string();
}

pub fn get_powered_number(any_number: i32, times_to_power: u32) -> i32 {
   return any_number.pow(times_to_power);
}
