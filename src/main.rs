use std::io;
use std::io::Write;
use reversing_string::init_reversing_string;
use crate::leap_year::init_leap_year;
use crate::verifying_allergies::init_verifying_allergies;

mod reversing_string;
mod leap_year;
mod verifying_allergies;
mod helper;

fn main() {
    choose_exercise();
}

fn choose_exercise() {
    let mut exercise_input = String::new();
    exercise_input = helper::get_user_input(exercise_input, "Exercise to be compiled".to_string());
    search_inputted_exercise(exercise_input.trim().to_string());
}

fn search_inputted_exercise(exercise_input: String) {
    let exercises_vector = vec!["reversing_string".to_string(), "leap_year".to_string(), "verifying_allergies".to_string()];

    if exercises_vector.contains(&exercise_input) {
        run_chosen_exercise(exercise_input, exercises_vector);
    } else {
        panic!("Exercise not found.");
    }
}

fn run_chosen_exercise(exercise_input: String, exercises_vector: Vec<String>) {
    if exercise_input.eq_ignore_ascii_case(&exercises_vector[0]) {
        init_reversing_string();
    } else if exercise_input.eq_ignore_ascii_case(&exercises_vector[1]) {
        init_leap_year();
    } else if exercise_input.eq_ignore_ascii_case(&exercises_vector[2]) {
        init_verifying_allergies();
    }
}
