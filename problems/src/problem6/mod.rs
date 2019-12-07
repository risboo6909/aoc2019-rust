use std::collections::HashMap;

use failure::{Error, format_err};
use utils::{split_by_lines, result, ProblemResult, RetOne};


struct Orbit {
    satellite: String,
    center: String,
}

fn traverse(map: &HashMap<&str, &str>, sat: &str) -> Vec<String> {

    let center = map[sat];
    if center == "COM" {
        return vec!["COM".to_owned()];
    }

    let mut xs = traverse(&map, center);
    xs.push(center.to_owned());

    xs
}

fn first_star(input: &[Orbit]) -> ProblemResult<usize> {

    let mut map = HashMap::<&str, &str>::new();
    for item in input {
        map.insert(&item.satellite, &item.center);
    }

    let mut total = 0;
    for (sat, _center) in map.iter() {
        total += traverse(&map, sat).len();
    }

    Ok(total)

}

fn second_star(input: &[Orbit]) -> ProblemResult<usize> {

    let mut map = HashMap::<&str, &str>::new();
    for item in input {
        map.insert(&item.satellite, &item.center);
    }

    // find first common point for both "YOU" and "SAN" to "COM"
    //
    // ["J", "E", "D", "C", "B", "COM"]
    // ["D", "C", "B", "COM"]
    //
    // in this example first common point is "D", count how many steps we need to get to "D"
    // from both sides and add them up - this will be the minimal path length we are looking for
    //
    for (idx1, your_step) in traverse(&map, map["YOU"]).iter().rev().enumerate() {
        for (idx2, p) in traverse(&map, map["SAN"]).iter().rev().enumerate() {
            if your_step == p {
                // +2 because indexing is starting from 0 but we have to count those ("J" and "D"
                // in the example above)s
                return Ok(idx1 + idx2 + 2)
            }
        }
    }

    Err(format_err!("Path not found"))

}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");

    let input: Vec<Orbit> = split_by_lines(input_raw, &|e: &str| {
        let tmp = e.split(')').collect::<Vec<&str>>();
        Ok(Orbit{
            center: tmp[0].to_owned(),
            satellite: tmp[1].to_owned(),
        })
    })?;

    Ok(result(first_star(&input), second_star(&input)))
}
