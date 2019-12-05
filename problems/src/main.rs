mod computer;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;

use failure::Error;
use std::fmt::Debug;
use colored::*;

use utils::Ret;

// problems
use crate::problem1 as p1;
use crate::problem2 as p2;
use crate::problem3 as p3;
use crate::problem4 as p4;
use crate::problem5 as p5;


fn exec<T: Debug>(f: &dyn Fn() -> Result<Ret<T>, Error>, problem_no: u32) {
    let result = f();
    match result {
        Err(err) => println!("{} {}:\n{}: {}\n", "problem".bold(),
                                  problem_no.to_string().bold(),
                                  "error".bold().red(), err),

        Ok(answer) => println!("{} {}:\n{}\n", "problem".bold(),
                               problem_no.to_string().bold(), answer),
    }
}

fn main() {
    println!("\n{}\n\n", "Advent of code 2019".bold());
    exec(&p1::solve, 1);
    exec(&p2::solve, 2);
    exec(&p3::solve, 3);
    exec(&p4::solve, 4);
    exec(&p5::solve, 5);
}
