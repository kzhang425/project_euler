use std::collections::HashMap;
use crate::getter::{get_user_input, get_function};

#[derive(Debug)]
pub struct Solutions {
    problems: HashMap<u16, String>,
    answers: HashMap<u16, i64>,
}

impl Solutions {
    pub fn new() -> Self {
        let problems = HashMap::new();
        let answers = HashMap::new();
        Solutions {
            problems,
            answers,
        }
    }

    pub fn store_problem(&mut self, prob_num: u16, sol: Option<(String, i64)>) {
        if let Some(tuple) = sol {
            self.problems.insert(prob_num, tuple.0);
            self.answers.insert(prob_num, tuple.1);
        }
    }

    fn show(&self, num: u16) {
        println!("{}", self.problems.get(&num).unwrap());
        println!("The solution is: {}", self.answers.get(&num).unwrap());
    }

    pub fn do_problem(&mut self) {
        loop {
            println!("Enter # of problem to view");
            
            // If the number is valid, should check if problem already solved
            if let Ok(value) = get_user_input() {
                // If problem has been done before, don't have to run the program again
                if self.problems.contains_key(&value) {
                    self.show(value);
                    break;
                }

                let problem_num = get_function(value);
                // If get_function returns None
                if problem_num == None {
                    println!("Problem does not currently exist!");
                    continue;
                }
                self.store_problem(value, problem_num);
                self.show(value);
                break;
            } else {
                println!("Invalid Input.");
                continue;
            }
        }
    }
}