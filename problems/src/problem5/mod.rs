use failure::{Error, format_err};

use utils::{result, ProblemResult, Ret, split_by_comma};
use crate::computer::Computer;

fn first_star(program: &[isize]) {
    let mut c = Computer::new();
}

pub(crate) fn solve() -> Result<Ret<isize>, Error> {
    let input_raw = include_str!("./input");
    let input = split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))?;

    first_star(&input);

    Ok(result(Ok(1),Ok(2)))
}
