use failure::Error;
use itertools::chain;

use utils::{result, split_digits, ProblemResult, RetTypes};

const LOWER_BOUND: usize = 178_416;
const UPPER_BOUND: usize = 676_461;

fn test_non_decr(xs: &[usize]) -> bool {
    for idx in 0..xs.len() {
        if idx > 0 && xs[idx] < xs[idx - 1] {
            return false;
        }
    }
    true
}

fn test_adj(xs: &[usize]) -> bool {
    // The idea is simple:
    // we pair list of elements with itself shifted by one, for example, for array [1,2,3,4,4,5]:
    //
    // [1,2,3,4,4,5]
    // [2,3,4,4,5]
    //
    // we consider pairs like (1, 2), (2, 3), (3, 4), (4, 4) and (4, 5)
    //
    // if there is at least one pair with equal elements, like (4, 4) we return true,
    // otherwise return false

    for (x, y) in xs.iter().zip(&xs[1..]) {
        if x == y {
            return true;
        }
    }
    false
}

fn test_adj_two(xs: &[usize]) -> bool {
    // This function is development of the idea introduced in function "test_adj"

    let mut count = 0u32;

    for (x, y) in xs.iter().zip(chain(&xs[1..], &[*xs.last().unwrap()])) {
        if x == y {
            count += 1;
        } else if count == 1 {
            return true;
        } else {
            count = 0;
        }
    }

    count == 2
}

fn pred(n: usize, p1: &dyn Fn(&[usize]) -> bool, p2: &dyn Fn(&[usize]) -> bool) -> bool {
    let digits = split_digits(n);
    p1(&digits) && p2(&digits)
}

fn first_star() -> ProblemResult<usize> {
    let counter = (LOWER_BOUND..=UPPER_BOUND)
        .filter_map(|n| {
            if pred(n, &test_non_decr, &test_adj) {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    Ok(counter)
}

fn second_star() -> ProblemResult<usize> {
    let counter = (LOWER_BOUND..=UPPER_BOUND)
        .filter_map(|n| {
            if pred(n, &test_non_decr, &test_adj_two) {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    Ok(counter)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    Ok(
        RetTypes::Usize(
            result(first_star(), second_star())
        )
    )
}
