use std::io;
use std::io::Write;

pub fn init_leap_year() {
    let mut inputted_year = String::new();
    print!("Input a year to verify if it's a leap year: ");
    io::stdout().flush().expect("Failed to flush.");
    io::stdin().read_line(&mut inputted_year).expect("Failed to read line.");

    if consist_leap_year(get_converted_inputted_year(inputted_year.trim().to_string())) {
        println!("It's a leap year!");
    } else {
        println!("Isn't a leap year!");
    }
}

fn consist_leap_year(converted_inputted_year: i32) -> bool {
    return converted_inputted_year % 4 == 0;
}

fn get_converted_inputted_year(inputted_year: String) -> i32 {
    let converted_inputted_year: i32 = inputted_year.parse().unwrap();
    return converted_inputted_year;
}
