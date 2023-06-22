use std::io;
use std::io::Write;
use reversing_string::init_reversing_string;
use crate::leap_year::init_leap_year;

mod reversing_string;
mod leap_year;

fn main() {
    choose_exercise();
}

fn choose_exercise() {
    let mut exercise_input = String::new();
    
    print!("Please, choose a exercise: ");
    io::stdout().flush().expect("Failed to flush.");
    io::stdin().read_line(&mut exercise_input).expect("Failed to read line.");
    search_inputted_exercise(exercise_input.trim().to_string());
}

fn search_inputted_exercise(exercise_input: String) {
    let exercises_vector = vec!["reversing_string".to_string(), "leap_year".to_string()];

    if exercises_vector.contains(&exercise_input) {
        run_chosen_exercise(exercise_input, exercises_vector)
    } else {
        panic!("Exercise not found.");
    }
}

fn run_chosen_exercise(exercise_input: String, exercises_vector: Vec<String>) {
    if exercise_input.eq(&exercises_vector[0]) {
        init_reversing_string();
    } else if exercise_input.eq(&exercises_vector[1]) {
        init_leap_year();
    }
}
