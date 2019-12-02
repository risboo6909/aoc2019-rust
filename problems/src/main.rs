mod problem1;
mod problem2;

use std::fmt::Debug;
use colored::*;

use utils::Ret;

// problems
use crate::problem1 as p1;
use crate::problem2 as p2;

fn exec<T: Debug>(f: &dyn Fn() -> Ret<T>, problem_no: u32) {
    let answer = f();
    println!("{} {}:\n{}\n", "problem".bold(), problem_no.to_string().bold(), answer);
}

fn main() {
    println!("\n{}\n\n", "Advent of code 2019".bold());
    exec(&p1::solve, 1);
    exec(&p2::solve, 2);
}
