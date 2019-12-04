use std::collections::{HashMap, HashSet};
use failure::format_err;

use utils::{split_by_lines, split_by_comma, result, man_dist_2d, ProblemResult, Ret, ParseResult};

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Op {
    dir: Dir,
    steps: u32,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Cursor {
    wire_len: u32,
    coords: Point,
}

fn new_cursor(x: i32, y: i32) -> Cursor {
    Cursor {
        wire_len: 0,
        coords: Point{x, y},
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

    Ok( Op{dir, steps: amount_str.parse::<u32>()?} )
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

fn make_moves(points: &mut HashMap<Point, HashSet<usize>>, lengths: &mut HashMap<(usize, Point), u32>,
              cursor: &mut Cursor, op: &Op, wire_no: usize) -> HashSet<Point> {

    let mut intersections = HashSet::new();

    for _ in 0..op.steps {

        advance_cursor(cursor, op.dir);

        points.entry(cursor.coords)
              .or_insert_with(HashSet::new)
              .insert(wire_no);

        lengths.entry((wire_no, cursor.coords))
               .or_insert(cursor.wire_len);

        if points[&cursor.coords].len() >= 2 {
            intersections.insert(cursor.coords);
        }

    }

    intersections
}

fn solve_both_stars(wires: &[Vec<Op>]) -> ProblemResult<(u32, u32)> {

    let mut points = HashMap::new();
    let mut lengths = HashMap::new();

    let mut overlaps: HashSet<Point> = HashSet::new();

    // assume central port is at (0, 0)
    for (wire_no, wire) in wires.iter().enumerate() {
        let mut cursor = new_cursor(0, 0);
        for ops in wire {
            overlaps.extend(
                &make_moves(&mut points, &mut lengths, &mut cursor, ops, wire_no)
            );
        }
    }

    // find intersection with the minimum Manhattan distance (part 1)

    let min_point = overlaps.iter()
        .min_by_key(|p| man_dist_2d(p.x, p.y, 0, 0));

    if min_point.is_none() {
        return Err(format_err!("Couldn't find appropriate solution!"));
    }

    let min_point = min_point.unwrap();

    // find intersection with the minimum wires length (part 2)

    let mut min_total_len = std::u32::MAX;

    for p in overlaps.iter() {
        let mut net_length = 0u32;
        for wire_no in 0..wires.len() {
            net_length += lengths[&(wire_no, *p)];
        }
        if min_total_len > net_length {
            min_total_len = net_length;
        }
    }

    Ok((man_dist_2d(min_point.x, min_point.y, 0, 0) as u32, min_total_len))
}

pub(crate) fn solve() -> ProblemResult<Ret<u32>> {

    let input_raw = include_str!("./input");

    let tmp: Result<Vec<_>, _> =
        split_by_lines(input_raw, &|e: &str| Ok(e.to_owned()))?
        .iter()
        .map(|line| split_by_comma(line, &|e: &str| make_pair(e)))
        .collect();

    let solutions = solve_both_stars(&tmp?)?;

    Ok(result(Ok(solutions.0), Ok(solutions.1)))
}
