use crate::helper;

pub fn init_armstrong_number() {
    let mut armstrong_number_str = String::new();
    armstrong_number_str = helper::get_user_input(armstrong_number_str, "Number to be checked".to_string());
    execute_armstrong_sum(armstrong_number_str);
}

fn execute_armstrong_sum(armstrong_number_str: String) {
    let armstrong_number_length: usize = armstrong_number_str.len();
    let mut final_armstrong_sum = 0;

    for char in armstrong_number_str.chars() {
        let converted_number = helper::get_converted_number_from_char(char);
        final_armstrong_sum = final_armstrong_sum +
            helper::get_powered_number(converted_number, helper::get_converted_u32_from_usize(armstrong_number_length));
    }
    check_armstrong_number(armstrong_number_str, final_armstrong_sum);
}

fn check_armstrong_number(armstrong_number_str: String, final_armstrong_sum: i32) {
    let armstrong_number_i32 = helper::get_converted_number_from_string(armstrong_number_str);

    if armstrong_number_i32 == final_armstrong_sum {
        println!("It's an armstrong number!");
    } else {
        println!("Isn't an armstrong number!");
    }
}
