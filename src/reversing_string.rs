use std::io;
use std::io::Write;
use crate::helper;

pub fn init_reversing_string() {
    let mut user_input = String::new();
    user_input = helper::get_user_input(user_input, "String to be reversed".to_string());
    println!("User input: {}", user_input);
    println!("User reversed input: {}", reversing_user_input(user_input));
}

fn reversing_user_input(user_input: String) -> String {
    return user_input.chars().rev().collect::<String>();
}
