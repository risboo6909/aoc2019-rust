use failure::{Error, format_err};

use crate::computer::Computer;
use utils::{split_by_comma, result, ProblemResult, RetOne};


fn first_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, vec![1]);
    c.step()?;
    Ok(c.get_output()?)
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, vec![2]);
    c.step()?;
    Ok(c.get_output()?)
}

pub(crate) fn solve() -> Result<RetOne<isize>, Error> {
    let input_raw = include_str!("./input");
    let input = split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))?;

    Ok(result(first_star(&input.clone()), second_star(&input.clone())))
}
