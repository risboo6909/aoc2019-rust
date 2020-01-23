use failure::Error;

use crate::computer::{parse_intcode, Computer};
use utils::{result, ProblemResult, RetTypes};

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

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    Ok(
        RetTypes::Isize(
            result(
                first_star(&input),
                second_star(&input),
            )
        )
    )

}
