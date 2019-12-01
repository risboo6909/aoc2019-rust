mod problem1;

use std::fmt::Display;
use colored::*;

use utils::ProblemResult;
use crate::problem1 as p1;

fn exec<T: Display>(f: &dyn Fn() -> ProblemResult<T>, problem_no: u32) {
    match f() {
        Ok(answer) => {
            println!("{} {}:\n{}", "problem".bold(), problem_no.to_string().bold(), answer);
        },
        Err(err) => {
            println!("{} {}:\n{}: {:?}, skipping",
                     "problem".bold(), problem_no.to_string().bold(),
                     "error: ".red(), err
            );
        }
    }
    println!("\n");
}

fn main() {
    println!("\n{}\n\n", "Advent of code 2019".bold());
    exec(&p1::solve, 1);
}
