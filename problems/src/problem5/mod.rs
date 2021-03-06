use failure::Error;
use utils::{result, ProblemResult, RetTypes};

use crate::computer::{consume_until_break, parse_intcode, Computer};

fn first_star(program: &[isize]) -> ProblemResult<Vec<isize>> {
    let mut c = Computer::new(program, Some(vec![1]));
    consume_until_break(&mut c)
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let mut c = Computer::new(program, Some(vec![5]));
    Ok(consume_until_break(&mut c).unwrap()[0])
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    let r1 = first_star(&input);
    let r2 = second_star(&input);

    assert_eq!(
        *r1.as_ref().unwrap(),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 6_731_945]
    );
    assert_eq!(*r2.as_ref().unwrap(), 9_571_668);

    Ok(
        RetTypes::VecIsizeIsize(
            result(r1, r2)
        )
    )
}
