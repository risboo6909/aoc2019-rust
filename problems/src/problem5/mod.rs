use failure::{Error, format_err};

use utils::{result, ProblemResult, Ret, split_by_comma};
use crate::computer::Computer;
use itertools::Itertools;


fn first_star(program: &[isize]) -> ProblemResult<String> {
    let mut c = Computer::new(program, 1);
    c.interpret()?;
    Ok(c.get_output().iter().join(","))
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, 5);
    c.interpret()?;
    Ok(c.get_output().pop_front().unwrap().parse()?)
}

pub(crate) fn solve() -> Result<Ret<String, isize>, Error> {
    let input_raw = include_str!("./input");
    let input = split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))?;

    Ok(result(first_star(&input.clone()), second_star(&input.clone())))
}
