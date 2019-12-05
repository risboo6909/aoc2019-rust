use failure::{Error, format_err};

use crate::computer::{Computer, Stdin};
use utils::{split_by_comma, result, ProblemResult, Ret};

fn first_star(program: &mut [u32]) -> ProblemResult<u32> {

    // input for the program
    program[1] = 12;
    program[2] = 2;

    let mut c = Computer::<u32>::new();

    // run the program
    c.interpret(program)
}

fn second_star(program: &mut [u32]) -> ProblemResult<u32> {

    let mut saved_program: Vec<u32> = vec![0; program.len()];
    let mut c = Computer::<u32>::new();

    saved_program.clone_from_slice(&program);

    for n in 0..10000 {

        let program = &mut saved_program.clone();

        let (noun, verb) = (n % 100u32, n / 100u32);

        // input for the program
        program[1] = noun;
        program[2] = verb;

        if c.interpret(program)? == 19_690_720 {
            return Ok(100 * noun + verb);
        }
    }

    Err(format_err!("Couldn't find appropriate solution!"))

}

pub(crate) fn solve() -> Result<Ret<u32>, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<u32> = split_by_comma(input_raw, &|e: &str| e.parse::<u32>()
        .or_else(|_| Err(format_err!("Failed to parse input"))))?;

    let r1 = first_star(&mut input.clone());
    let r2 = second_star(&mut input.clone());

    assert_eq!(*r1.as_ref().unwrap(), 3706713);
    assert_eq!(*r2.as_ref().unwrap(), 8609);

    Ok(result(r1, r2))
}
