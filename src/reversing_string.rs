use std::io;
use std::io::Write;

pub fn init_reversing_string() {
    let mut user_input = String::new();
    user_input = getting_user_input(user_input.clone());
    print!("User input: {}", user_input);
    print!("User reversed input: {}", reversing_user_input(user_input));
}

fn getting_user_input(mut user_input: String) -> String {
    print!("Please, consider inputting a word: ");
    io::stdout().flush().expect("Failed to flush.");
    io::stdin().read_line(&mut user_input).expect("Failed to read line.");
    return user_input;
}

fn reversing_user_input(user_input: String) -> String {
    return user_input.chars().rev().collect::<String>();
}
