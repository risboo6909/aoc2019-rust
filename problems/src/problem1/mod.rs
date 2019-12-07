use failure::{Error, format_err};
use utils::{split_by_lines, result, ProblemResult, RetOne};

fn fuel_req(mass: u32) -> u32 {
    let a = mass / 3;
    if a >= 2 {
        a - 2
    } else {
        0
    }
}

fn first_star(input: &[u32]) -> ProblemResult<u32> {
    let mut fuel = 0;
    for mass in input {
        fuel += fuel_req(*mass);
    }
    Ok(fuel)
}

fn second_star(input: &[u32]) -> ProblemResult<u32> {
    let mut fuel = 0;

    for mass in input {
        let mut mass_remain = *mass;
        while mass_remain > 0 {
            mass_remain = fuel_req(mass_remain);
            fuel += mass_remain;
        }
    }

    Ok(fuel)
}

pub(crate) fn solve() -> Result<RetOne<u32>, Error> {
    let input_raw = include_str!("./input");
    let input: Vec<u32> = split_by_lines(input_raw, &|e: &str|
        e.parse::<u32>().or_else(|_| Err(format_err!("Failed to parse input"))))?;

    Ok(result(first_star(&input), second_star(&input)))
}
