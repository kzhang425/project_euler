use crate::problems::*;
use std::error::Error;

pub fn get_function(num: u16) -> Option<(String, u64)> {
    match num {
        // Update for problems
        1 => Some(problem1::solve()),
        2 => Some(problem2::solve()),
        3 => Some(problem3::solve()),
        4 => Some(problem4::solve()),
        5 => Some(problem5::solve()),
        6 => Some(problem6::solve()),
        7 => Some(problem7::solve()),
        8 => Some(problem8::solve()),
        9 => Some(problem9::solve()),
        10 => Some(problem10::solve()),
        11 => Some(problem11::solve()),
        12 => Some(problem12::solve()),
        13 => Some(problem13::solve()),
        14 => Some(problem14::solve()),
        15 => Some(problem15::solve()),
        _ => None,
    }
}

pub fn get_user_input() -> Result<u16, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    // Use the string to generate a u16
    let trimmed = input.trim().parse::<u16>()?;
    Ok(trimmed)
}

