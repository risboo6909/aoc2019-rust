use std::fmt::{self, Display, Formatter};
use std::error::Error;
use colored::*;

pub struct Ret<T: Display> {
    answer_basic: T,
    answer_adv: T,
}

impl<T: Display> Display for Ret<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}: {}\n{}: {}",
            "first star solution".blue(),
            self.answer_basic,
            "second star solution".yellow(),
            self.answer_adv
        )
    }
}


pub type ProblemResult<T> = Result<Ret<T>, Box<dyn Error>>;

pub fn ok_result<T: Display>(basic: T, adv: T) -> ProblemResult<T> {
    Ok(
        Ret {
            answer_basic: basic,
            answer_adv: adv,
        }
    )
}

pub fn split_by_lines<T>(input: &str, f: &dyn Fn(&str) -> T) -> Vec<T> {
    input.split('\n')
         .filter(|item|
             if item != &"" {
                 true
             } else {
                 false
             }
         )
         .map(|item| f(item))
         .collect::<Vec<T>>()
}
