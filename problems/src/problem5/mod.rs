use failure::{Error, format_err};
use utils::{result, ProblemResult, Ret, split_by_comma};

use crate::computer::{Computer, consume_until_break};

fn first_star(program: &[isize]) -> ProblemResult<Vec<isize>> {
    let mut c = Computer::new(program, vec![1]);
    consume_until_break(&mut c)
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, vec![5]);
    c.step()?;

    Ok(c.get_output()?)
}

pub(crate) fn solve() -> Result<Ret<Vec<isize>, isize>, Error> {
    let input_raw = include_str!("./input");
    let input = split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))?;

    let r1 = first_star(&input.clone());
    let r2 = second_star(&input.clone());

    assert_eq!(*r1.as_ref().unwrap(), vec![0,0,0,0,0,0,0,0,0,6_731_945]);
    assert_eq!(*r2.as_ref().unwrap(), 9_571_668);

    Ok(result(r1, r2))
}
