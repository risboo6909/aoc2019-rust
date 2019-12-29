mod computer;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod problem10;
mod problem11;
mod problem12;
mod problem13;
mod problem14;

use failure::Error;
use std::{fmt::Debug, time::SystemTime};
use colored::*;

use utils::Ret;

// problems
use crate::problem1 as p1;
use crate::problem2 as p2;
use crate::problem3 as p3;
use crate::problem4 as p4;
use crate::problem5 as p5;
use crate::problem6 as p6;
use crate::problem7 as p7;
use crate::problem8 as p8;
use crate::problem9 as p9;
use crate::problem10 as p10;
use crate::problem11 as p11;
use crate::problem12 as p12;
use crate::problem13 as p13;
use crate::problem14 as p14;


fn exec<T: Debug, K: Debug>(f: &dyn Fn() -> Result<Ret<T, K>, Error>, problem_no: u32) {

    let now = SystemTime::now();
    let result = f();
    let elapsed = now.elapsed().unwrap().as_millis();

    match result {
        Err(err) => println!("{} {}:\n{}: {}", "problem".bold(),
                                  problem_no.to_string().bold(),
                                  "error".bold().red(), err),

        Ok(answer) => println!("{} {}:\n{}", "problem".bold(),
                                 problem_no.to_string().bold(), answer),
    }

    println!("time elapsed for problem: {} millis\n", elapsed);

}

fn main() {
    println!("\n{}\n\n", "Advent of code 2019".bold());

    let now = SystemTime::now();

    exec(&p1::solve, 1);
    exec(&p2::solve, 2);
    exec(&p3::solve, 3);
    exec(&p4::solve, 4);
    exec(&p5::solve, 5);
    exec(&p6::solve, 6);
    exec(&p7::solve, 7);
    exec(&p8::solve, 8);
    exec(&p9::solve, 9);
    exec(&p10::solve, 10);
    exec(&p11::solve, 11);
    exec(&p12::solve, 12);
    exec(&p13::solve, 13);
    exec(&p14::solve, 14);

    println!("{} {} {}", "Total time taken:".bold(),
             now.elapsed().unwrap().as_millis().to_string().bold().green(),
             "millis".bold());
}
