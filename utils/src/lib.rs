use std::fmt::{self, Display, Debug, Formatter};
use num;
use num_traits::sign::Signed;
use colored::*;
use failure::Error;
use failure::_core::ops::{Add, Sub};
use num_traits::FromPrimitive;

pub type ProblemResult<T> = Result<T, Error>;
pub type ParseResult<T> = Result<T, Error>;

pub struct Ret<T> {
    answer_basic: ProblemResult<T>,
    answer_adv: ProblemResult<T>,
}

impl<T: Debug> Display for Ret<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}: {:?}\n{}: {:?}",
            "first star solution".blue(),
            self.answer_basic,
            "second star solution".yellow(),
            self.answer_adv
        )
    }
}

pub fn man_dist_2d<T: Add<Output=T> + Sub<Output=T> + Signed>(x1: T, y1: T, x2: T, y2: T) -> T {
    num::abs(x1 - x2) + num::abs(y1 - y2)
}

pub fn result<T: Debug>(basic: ProblemResult<T>, adv: ProblemResult<T>) -> Ret<T> {
    Ret {
        answer_basic: basic,
        answer_adv: adv,
    }
}

pub fn split_by_lines<T>(input: &str, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    split_by(input, '\n', f)
}

pub fn split_by_comma<T>(input: &str, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    split_by(input, ',', f)
}

pub fn split_by<T>(input: &str, sep: char, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    let res: ParseResult<Vec<_>> = input.split(sep)
         .filter(|item|
             if item != &"" {
                 true
             } else {
                 false
             }
         )
         .map(|item| f(item))
         .collect();

    res
}
