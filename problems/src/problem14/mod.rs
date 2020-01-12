use failure::Error;
use std::{collections::HashMap, fmt};

use itertools::Itertools;
use std::collections::hash_map::Entry;

use utils::{result, split_by_lines, ParseResult, ProblemResult, RetOne};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Term {
    coeff: usize,
    label: String,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.coeff, self.label)
    }
}

#[derive(Debug, Clone)]
struct Formulae {
    left: Vec<Term>,
    right: Term,
}

impl fmt::Display for Formulae {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.left.iter().join(", "))
    }
}

#[derive(Debug, Copy, Clone)]
struct OreItems {
    items: usize,
    ore: usize,
}

fn make_dep_map(formulas: &[Formulae]) -> HashMap<String, Formulae> {
    let mut res: HashMap<String, Formulae> = HashMap::new();
    for f in formulas {
        res.insert(f.right.label.to_owned(), f.clone());
    }
    res
}

fn recognize_term(term: &str) -> ParseResult<Term> {
    let parts: Vec<&str> = term.split(' ').collect();

    Ok(Term {
        coeff: parts[0].parse::<usize>()?,
        label: parts[1].to_owned(),
    })
}

fn parse_line(line: &str) -> ParseResult<Formulae> {
    let parts: Vec<&str> = line.split("=>").collect();

    let left_part: ParseResult<Vec<_>> = parts[0]
        .split(',')
        .map(|t| recognize_term(t.trim()))
        .collect();

    let right_part = recognize_term(parts[1].trim())?;

    Ok(Formulae {
        left: left_part?,
        right: right_part,
    })
}

fn ore_required(
    entry: Entry<String, usize>,
    items_req: usize,
    items_from_ore: usize,
    ore: usize,
) -> usize {
    let r = entry.or_insert(0);

    if *r >= items_req {
        *r -= items_req;
        0
    } else {
        *r = items_from_ore - items_req - *r;
        ore
    }
}

fn rec(
    dep_map: &HashMap<String, Formulae>,
    reserve: &mut HashMap<String, usize>,
    f: &Formulae,
    amount: usize,
) -> OreItems {
    // dep_map - maps term equation required to produce it
    // table - contains terms we already know prices for (to reduce recursive search)

    let mut total_ore = 0;

    let k = (amount as f64 / f.right.coeff as f64).ceil() as usize;

    // the equation will be like: "N XXXX -> M ORE", meaning that we must take M of ORE,
    // to produce N of XXXX

    if &f.left[0].label == "ORE" {
        return OreItems {
            items: k * f.right.coeff,
            ore: k * f.left[0].coeff,
        };
    }

    for term in f.left.iter() {
        let r = reserve.entry(term.label.clone()).or_insert(0);

        let need = if *r >= k * term.coeff {
            *r -= k * term.coeff;
            0
        } else {
            let tmp = *r;
            *r = 0;
            k * term.coeff - tmp
        };

        let p = rec(dep_map, reserve, &dep_map[&term.label], need);

        total_ore += ore_required(reserve.entry(term.label.clone()), need, p.items, p.ore);
    }

    OreItems {
        items: k * f.right.coeff,
        ore: total_ore,
    }
}

fn compute_ore_consumption(
    dep_map: &HashMap<String, Formulae>,
    reserve: &mut HashMap<String, usize>,
    amount: usize,
) -> OreItems {
    rec(dep_map, reserve, &dep_map["FUEL"], amount)
}

fn first_star(dep_map: &HashMap<String, Formulae>) -> ProblemResult<usize> {
    Ok(compute_ore_consumption(dep_map, &mut HashMap::new(), 1).ore)
}

fn find_max(dep_map: &HashMap<String, Formulae>, mut cur_fuel: usize, max_ore: usize) -> usize {
    let mut reserve = HashMap::<String, usize>::new();
    let mut step = 1;

    loop {
        let res = compute_ore_consumption(&dep_map, &mut reserve, cur_fuel);

        if res.ore >= max_ore {
            break res.items;
        }

        step *= 2;
        cur_fuel += step;

        reserve.clear();
    }
}

fn second_star(dep_map: &HashMap<String, Formulae>) -> ProblemResult<usize> {
    let ore_avail = 1_000_000_000_000usize;
    let mut reserve = HashMap::<String, usize>::new();

    let mut min_fuel =
        ore_avail / compute_ore_consumption(dep_map, &mut HashMap::<String, usize>::new(), 1).ore;
    let mut max_fuel = find_max(dep_map, min_fuel, ore_avail);

    loop {
        let cur_fuel = min_fuel + (max_fuel - min_fuel) / 2;

        let res = compute_ore_consumption(&dep_map, &mut reserve, cur_fuel);

        if res.ore == ore_avail {
            return Ok(cur_fuel);
        } else if res.ore > ore_avail {
            if compute_ore_consumption(&dep_map, &mut reserve, cur_fuel - 1).ore <= ore_avail {
                return Ok(cur_fuel - 1);
            }

            max_fuel = res.items;
        } else if res.ore < ore_avail {
            if compute_ore_consumption(&dep_map, &mut reserve, cur_fuel + 1).ore >= ore_avail {
                return Ok(cur_fuel);
            }

            min_fuel = res.items;
        }

        reserve.clear();
    }
}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");

    let formulas = split_by_lines(input_raw, &parse_line)?;
    let dep_map = make_dep_map(&formulas);

    Ok(result(first_star(&dep_map), second_star(&dep_map)))
}
