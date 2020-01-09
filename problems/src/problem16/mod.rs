use std::cmp::min;

use failure::Error;

use utils::{result, ProblemResult, RetOne, make_number, number_to_string};

const BASE_PAT: [isize; 4] = [0, 1, 0, -1];


fn split_to_digits(input: &str) -> Vec<isize> {
    let mut digits = Vec::new();
    for d in input.chars() {
        digits.push(d.to_digit(10).unwrap() as isize);
    }

    digits
}

fn get_pat_by_idx(pos: usize, idx: usize) -> isize {
    let pat_idx = idx % ((pos + 1) * BASE_PAT.len()) / (pos + 1);
    BASE_PAT[pat_idx]
}

fn multiply(xs: &[isize], sums: &[isize], group_len: usize) -> isize {

    let mut net = 0;
    let mut idx = min(group_len - 1, 0);

    while idx < xs.len() {

        let y = get_pat_by_idx(group_len, idx + 1);

        if group_len > 0 {

            let delta = min(
                if idx == 0 {
                    group_len
                } else {
                    group_len + 1
                },
                xs.len() - idx,
            );

            net += if y > 0 {
                sums[idx + delta - 1] - sums[idx - 1]
            } else if y < 0 {
                -(sums[idx + delta - 1] - sums[idx - 1])
            } else {
                0
            };

            idx += delta;

        } else {

            net += xs[idx] * y;
            idx += 2;

        }

    }

    net
}

fn make_part_sums(digits: &[isize], sums: &mut Vec<isize>) {
    for idx in 0..digits.len() {
        sums[idx] = digits[idx] + if idx == 0 { 0 } else { sums[idx - 1] };
    }
}

fn compute_n_phases(input: &str, phases: usize) -> Vec<isize> {

    let mut digits = split_to_digits(input);
    let mut sums = vec![0; digits.len()];

    for _ in 0..phases {

        let mut pos = 0;

        make_part_sums(&digits, &mut sums);

        while pos < digits.len() {
            digits[pos] = multiply(&digits, &sums, pos).abs() % 10;
            pos += 1;
        }

    }

    digits
}


fn first_star(input: &str, phases: usize) -> ProblemResult<String> {

    let tmp =
        compute_n_phases(input, phases)
        .iter()
        .take(8)
        .copied()
        .collect::<Vec<isize>>();

    Ok(number_to_string(&tmp))

}

fn second_star(input: &str, phases: usize, times: usize) -> ProblemResult<String> {

    let offset: isize = make_number(
        &split_to_digits(input)
        .iter()
        .take(7)
        .copied()
        .collect::<Vec<isize>>()
    );

    let repeated = input.repeat(times);

    let tmp =
        compute_n_phases(&repeated, phases)
            .iter()
            .skip(offset as usize)
            .take(8)
            .copied()
            .collect::<Vec<isize>>();

    Ok(number_to_string(&tmp))
}

pub(crate) fn solve() -> Result<RetOne<String>, Error> {
    let input_raw = include_str!("./input");
    let input = input_raw.trim();

    let r1 = first_star(input, 100);
    // answer must be a string because of possible leading zeros
    assert_eq!(*r1.as_ref().unwrap(), "36627552");

    let r2 = second_star(input, 100, 10000);
    assert_eq!(*r2.as_ref().unwrap(), "79723033");

    Ok(result(r1, r2))
}
