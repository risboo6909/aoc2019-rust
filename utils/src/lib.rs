use std::fmt::{self, Display, Debug, Formatter};
use core::ops::{Add, Sub};

use colored::*;
use failure::Error;
use num_traits::{Num, CheckedDiv, sign::Signed};
use num::FromPrimitive;

pub type ProblemResult<T> = Result<T, Error>;
pub type ParseResult<T> = Result<T, Error>;

pub struct Ret<T, K> {
    answer_basic: ProblemResult<T>,
    answer_adv: ProblemResult<K>,
}

pub type RetOne<T> = Ret<T, T>;

impl<T: Debug, K: Debug> Display for Ret<T, K> {
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

pub fn result<T: Debug, K: Debug>(basic: ProblemResult<T>, adv: ProblemResult<K>) -> Ret<T, K> {
    Ret {
        answer_basic: basic,
        answer_adv: adv,
    }
}

pub fn split_by_lines<T>(input: &str, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    split_by(input, "\n", f)
}

pub fn split_by_comma<T>(input: &str, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    split_by(input, ",", f)
}

pub fn split_by<T>(input: &str, sep: &str, f: &dyn Fn(&str) -> ParseResult<T>) -> ParseResult<Vec<T>> {
    let res: ParseResult<Vec<_>> = input.split(sep)
         .filter(|item| item != &"")
         .map(|item| f(item))
         .collect();

    res
}

pub fn split_digits<T: Copy + Clone + FromPrimitive + CheckedDiv<Output=T> + Num>(n: T) -> Vec<T> {
    if n == T::zero() {
        return Vec::new();
    }

    let mut res = split_digits(n / T::from_u8(10).unwrap());
    res.push(n % (T::from_u8(10).unwrap()));

    res
}
