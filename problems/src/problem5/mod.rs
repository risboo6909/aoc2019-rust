use failure::Error;
use utils::{result, ProblemResult, Ret};

use crate::computer::{Computer, consume_until_break, parse_intcode};

fn first_star(program: &[isize]) -> ProblemResult<Vec<isize>> {
    let mut c = Computer::new(program, Some(vec![1]));
    consume_until_break(&mut c)
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, Some(vec![5]));
    Ok(consume_until_break(&mut c).unwrap()[0])
}

pub(crate) fn solve() -> Result<Ret<Vec<isize>, isize>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    let r1 = first_star(&input.clone());
    let r2 = second_star(&input.clone());

    assert_eq!(*r1.as_ref().unwrap(), vec![0,0,0,0,0,0,0,0,0,6_731_945]);
    assert_eq!(*r2.as_ref().unwrap(), 9_571_668);

    Ok(result(r1, r2))
}
