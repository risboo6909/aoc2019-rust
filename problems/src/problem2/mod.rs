use failure::{format_err, Error};

use crate::computer::{parse_intcode, Computer};
use utils::{result, ProblemResult, RetTypes};

fn first_star(program: &mut [isize]) -> ProblemResult<isize> {
    // input for the program
    program[1] = 12;
    program[2] = 2;

    let mut c = Computer::new(program, None);

    // run the program
    c.step()?;

    Ok(c.get_cell(0))
}

fn second_star(program: &mut [isize]) -> ProblemResult<isize> {
    let mut saved_program: Vec<isize> = vec![0; program.len()];

    saved_program.clone_from_slice(&program);

    for n in 0..10000isize {
        let program = &mut saved_program.clone();

        let (noun, verb) = (n % 100, n / 100);

        // input for the program
        program[1] = noun;
        program[2] = verb;

        let mut c = Computer::new(program, None);

        c.step()?;

        if c.get_cell(0) == 19_690_720 {
            return Ok(100 * noun + verb);
        }
    }

    Err(format_err!("Couldn't find appropriate solution!"))
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let mut input = parse_intcode(input_raw)?;

    let r1 = first_star(&mut input);
    let r2 = second_star(&mut input);

    assert_eq!(*r1.as_ref().unwrap(), 3_706_713);
    assert_eq!(*r2.as_ref().unwrap(), 8609);

    Ok(
        RetTypes::Isize(
            result(r1, r2)
        )
    )
}
