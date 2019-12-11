use std::fmt::{self, Display, Debug, Formatter};
use core::ops::{Add, Sub, Mul, Div, AddAssign};

use colored::*;
use failure::Error;
use num_traits::{Num, CheckedDiv, sign::Signed};
use num::FromPrimitive;
use failure::_core::iter::FromIterator;

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

pub fn dot_product<T: Add<Output=T> + Mul<Output=T> + AddAssign + Num + Copy>(xs: &[T], ys: &[T]) -> T {
    let mut net: T = T::zero();
    for (x, y) in xs.iter().zip(ys) {
        net += *x * *y;
    }
    net
}

pub fn dot_product_2d<T: Add<Output=T> + Mul<Output=T> + AddAssign + Num + Copy>(x1: T, y1: T, x2: T, y2: T) -> T {
    dot_product(&vec![x1, y1], &vec![x2, y2])
}

pub fn len<T: Add<Output=T> + Mul<Output=T> + AddAssign + Num + Copy>(coords: &[T]) -> f64 where f64: From<T> {
    let mut net: T = T::zero();

    for c in coords {
        net += *c * *c;
    }

    f64::from(net).sqrt()
}

pub fn normalize<T: Div<Output=T> + AddAssign + Num + Copy>(coords: &[T]) -> Vec<f64> where f64: From<T> {
    let l = len(coords);
    let mut new_coords = Vec::new();

    for c in coords {
        new_coords.push(f64::from(*c) / l);
    }

    new_coords
}

pub fn normalize_2d<T: Div<Output=T> + AddAssign + Num + Copy>(coords: (T, T)) -> (f64, f64) where f64: From<T> {
    let n = normalize(&vec![coords.0, coords.1]);
    (n[0], n[1])
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
