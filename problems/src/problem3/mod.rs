use failure::{format_err, Error};
use std::collections::{HashMap, HashSet};

use utils::{
    man_dist_2d, result, split_by_comma, split_by_lines, ParseResult, ProblemResult, RetTypes
};

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Op {
    dir: Dir,
    steps: usize,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Cursor {
    wire_len: usize,
    coords: Point,
}

fn new_cursor(x: isize, y: isize) -> Cursor {
    Cursor {
        wire_len: 0,
        coords: Point { x, y },
    }
}

fn make_pair(item: &str) -> ParseResult<Op> {
    let (dir_str, amount_str) = (&item[..1], &item[1..]);

    let dir = match dir_str {
        "L" => Dir::Left,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "D" => Dir::Down,
        _ => {
            return Err(format_err!("Invalid input {}", item));
        }
    };

    Ok(Op {
        dir,
        steps: amount_str.parse::<usize>()?,
    })
}

fn advance_cursor(mut cursor: &mut Cursor, dir: Dir) {
    match dir {
        Dir::Left => cursor.coords.x -= 1,
        Dir::Right => cursor.coords.x += 1,
        Dir::Up => cursor.coords.y += 1,
        Dir::Down => cursor.coords.y -= 1,
    }
    cursor.wire_len += 1;
}

fn make_moves(
    points: &mut HashMap<Point, HashSet<usize>>,
    lengths: &mut HashMap<(usize, Point), usize>,
    cursor: &mut Cursor,
    op: &Op,
    wire_no: usize,
) -> HashSet<Point> {
    let mut intersections = HashSet::new();

    for _ in 0..op.steps {
        advance_cursor(cursor, op.dir);

        points
            .entry(cursor.coords)
            .or_insert_with(HashSet::new)
            .insert(wire_no);

        lengths
            .entry((wire_no, cursor.coords))
            .or_insert(cursor.wire_len);

        if points[&cursor.coords].len() >= 2 {
            intersections.insert(cursor.coords);
        }
    }

    intersections
}

fn solve_both_stars(wires: &[Vec<Op>]) -> ProblemResult<(usize, usize)> {
    let mut points = HashMap::new();
    let mut lengths = HashMap::new();

    let mut overlaps: HashSet<Point> = HashSet::new();

    // assume central port is at (0, 0)
    for (wire_no, wire) in wires.iter().enumerate() {
        let mut cursor = new_cursor(0, 0);
        for ops in wire {
            overlaps.extend(&make_moves(
                &mut points,
                &mut lengths,
                &mut cursor,
                ops,
                wire_no,
            ));
        }
    }

    // find intersection with the minimum Manhattan distance (part 1)

    let min_point = overlaps.iter().min_by_key(|p| man_dist_2d(p.x, p.y, 0, 0));

    if min_point.is_none() {
        return Err(format_err!("Couldn't find appropriate solution!"));
    }

    // find intersection with the minimum wires length (part 2)

    let mut min_total_len = std::usize::MAX;

    for p in overlaps.iter() {
        let net_length = (0..wires.len())
            .map(|wire_no| lengths[&(wire_no, *p)])
            .sum();

        if min_total_len > net_length {
            min_total_len = net_length;
        }
    }

    let min_point = min_point.unwrap();

    Ok((
        man_dist_2d(min_point.x, min_point.y, 0, 0) as usize,
        min_total_len,
    ))
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");

    let tmp: Result<Vec<_>, _> = split_by_lines(input_raw, &|e: &str| Ok(e.to_owned()))?
        .iter()
        .map(|line| split_by_comma(line, &|e: &str| make_pair(e)))
        .collect();

    let solutions = solve_both_stars(&tmp?)?;

    Ok(
        RetTypes::Usize(
            result(Ok(solutions.0), Ok(solutions.1))
        )
    )
}
