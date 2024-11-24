use std::io;

use crate::nsm_save_data::nsm_save::is_valid_nsm_save_name;

pub fn get_valid_nsm_save_name(prompt: &str) -> String {
    let mut user_input: String = String::new();
    loop {
        // get input
        println!("{prompt}");
        io::stdin().read_line(&mut user_input).expect("Couldn't get input!");
        // format input
        user_input = user_input.trim().to_string();
        // First make sure the given name is valid
        if !is_valid_nsm_save_name(&user_input) {
            println!("Invalid name. Please make sure name only contains characters A-Z, a-z, 0-9, -, _");
            continue;
        }
        break;
    }
    user_input
}

pub fn get_yes_or_no(prompt: &str) -> bool {
    let mut user_input: String = String::new();
    loop {
        // get input
        println!("{prompt}");
        io::stdin().read_line(&mut user_input).expect("Couldn't get input!");
        // format input
        user_input = user_input.trim().to_lowercase().to_string();
        if user_input == "y" || user_input == "yes" {
            return true;
        } else if user_input == "n" || user_input == "no" {
            return false
        } else {
            println!("Invalid input: Please enter \'y\'/\"yes\" or \'n\'/\"no\"");
            continue;
        }
    }
}

