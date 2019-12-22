use std::{fmt, collections::HashMap};
use failure::Error;

use utils::{split_by_lines, result, ProblemResult, ParseResult, RetOne,};
use num_integer::Integer;
use image::imageops::contrast;
use std::collections::hash_map::Entry;
use itertools::Itertools;


#[derive(Debug, Hash, PartialEq, Eq)]
struct Term {
    coeff: usize,
    label: String,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.coeff, self.label)
    }
}

#[derive(Debug)]
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
struct Price {
    items: usize,
    ore: usize,
}

fn make_dep_map(formulas: &[Formulae]) -> HashMap<String, &Formulae> {
    let mut res: HashMap<String, &Formulae> = HashMap::new();
    for f in formulas {
        res.insert(f.right.label.to_owned(), f);
    }
    res
}

fn recognize_term(term: &str) -> ParseResult<Term> {
    let parts: Vec<&str> = term.split(" ").collect();

    Ok(Term {
        coeff: parts[0].parse::<usize>()?,
        label: parts[1].to_owned(),
    })
}

fn parse_line(line: &str) -> ParseResult<Formulae> {
    let parts: Vec<&str> = line.split("=>").collect();

    let left_part: ParseResult<Vec<_>> = parts[0].split(",")
                                                 .map(|t| recognize_term(t.trim()))
                                                 .collect();

    let right_part = recognize_term(parts[1].trim())?;

    Ok(Formulae {
        left: left_part?,
        right: right_part,
    })
}

fn ore_required(entry: Entry<String, usize>, mut items_req: usize, items_from_ore: usize, ore: usize) -> (usize, usize) {

    let r = entry.or_insert(0);

    // we have more items reserved than we need
    if *r >= items_req {
        *r -= items_req;
        (0, items_req)
    } else {
        // items must be generated from ore
        if items_from_ore >= items_req {
            // enough items to produce from ore
            *r = items_from_ore - items_req - *r;
            (ore, items_req)
        } else {
            // no enough items from ore to generate a new one, add items from the reserve too
            let tmp = *r;
            *r = 0;
            (ore, items_from_ore + tmp)
        }
    }

}

fn rec(dep_map: &HashMap<String, &Formulae>, reserve: &mut HashMap<String, usize>, f: &Formulae) -> Price {

    // dep_map - maps term equation required to produce it
    // table - contains terms we already know prices for (to reduce recursive search)

    let mut total_ore = 0;

    // the equation will be like: "N XXXX -> M ORE", meaning that we must take M of ORE,
    // to produce N of XXXX
    if &f.left[0].label == "ORE" {
        let p = Price {
            items: f.right.coeff,
            ore: f.left[0].coeff,
        };
        return p;
    }

    for term in f.left.iter() {

        let r = reserve.entry(term.label.clone()).or_insert(0);

        // subtract matter we already have reserved
        let mut needed = if *r >= term.coeff {
            *r -= term.coeff;
            0
        } else {
            let tmp = *r;
            *r = 0;
            term.coeff - tmp
        };

        // compute rest ore
        while needed > 0 {
            let p = rec(dep_map, reserve, dep_map[&term.label]);
            let (ore, produced) = ore_required(
                reserve.entry(term.label.clone()), needed, p.items, p.ore
            );

            needed -= produced;
            total_ore += ore;
        }

    }

    Price {
        items: f.right.coeff,
        ore: total_ore,
    }

}

fn first_star(dep_map: &HashMap<String, &Formulae>) -> ProblemResult<usize> {

    // start from FUEL
    Ok(rec(dep_map, &mut HashMap::new(), dep_map["FUEL"]).ore)
}

fn second_star(dep_map: &HashMap<String, &Formulae>) -> ProblemResult<usize> {
    Ok(rec(dep_map, &mut HashMap::new(), dep_map["FUEL"]).ore)
}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");

    let formulas = split_by_lines(input_raw, &parse_line)?;
    let dep_map = make_dep_map(&formulas);

    Ok(result(Ok(first_star(&dep_map)?), Ok(2)))
}
