use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::iter::Iterator;
use crate::helper;

const ALLERGIES_DATA: HashMap<&str, i32> =
    [
        ("eggs", 1),
        ("peanuts", 2),
        ("shellfish", 4),
        ("strawberries", 8),
        ("tomatoes", 16),
        ("chocolate", 32),
        ("pollen", 64),
        ("cats", 128)
    ].iter().cloned().collect();

pub fn init_verifying_allergies() {
    let mut allergic_score = String::new();
    let mut allergic_score_converted;
    allergic_score = helper::get_user_input(allergic_score, "Allergic Score to be checked".to_string());
    allergic_score_converted = helper::get_converted_number_from_string(allergic_score);
}
