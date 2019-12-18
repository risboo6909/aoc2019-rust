use failure::Error;

use crate::computer::{Computer, parse_intcode};
use utils::{result, ProblemResult, RetOne};


fn first_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, Some(vec![1]));
    c.step()?;
    Ok(c.get_output()?)
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, Some(vec![2]));
    c.step()?;
    Ok(c.get_output()?)
}

pub(crate) fn solve() -> Result<RetOne<isize>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;
    Ok(result(first_star(&input.clone()), second_star(&input.clone())))
}
