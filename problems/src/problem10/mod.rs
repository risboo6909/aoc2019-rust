use std::fmt;
use std::collections::{HashSet, VecDeque};

use failure::Error;

use utils::{split_by, split_by_lines, result, ProblemResult, RetOne, dot_product_2d, vec_product_2d, normalize_2d};

const PREC: f64 = 0.999_999_999_99;
const TO_DESTROY: usize = 200;

type Coords<T> = (T, T);

#[derive(Debug)]
struct AngleCoords {
    angle: f64,
    board_coords: Coords<usize>,
    coords: Coords<f64>,
}

#[derive(Copy, Clone, PartialEq)]
enum Point {
    Asteroid,
    Empty,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Point::Asteroid => write!(f, "A"),
            Point::Empty => write!(f, "E"),
        }
    }
}

fn get_vicinity(x_idx: usize, y_idx: usize, x_max: usize, y_max: usize) -> Vec<Coords<usize>> {

    let mut v = Vec::new();

    //       3 4 5
    //        \|/
    //      1- * -2
    //        /|\
    //       6 7 8

    // 1
    if x_idx > 0 {
        v.push((x_idx - 1, y_idx))
    }

    // 2
    if x_idx < x_max - 1 {
        v.push((x_idx + 1, y_idx))
    }

    // 3
    if x_idx > 0 && y_idx > 0 {
        v.push((x_idx - 1, y_idx - 1))
    }

    // 4
    if y_idx > 0 {
        v.push((x_idx, y_idx - 1))
    }

    // 5
    if x_idx < x_max - 1 && y_idx > 0 {
        v.push((x_idx + 1, y_idx - 1))
    }

    // 6
    if x_idx > 0 && y_idx < y_max - 1 {
        v.push((x_idx - 1, y_idx + 1))
    }

    // 7
    if y_idx < y_max - 1 {
        v.push((x_idx, y_idx + 1))
    }

    // 8
    if x_idx < x_max - 1 && y_idx < y_max - 1 {
        v.push((x_idx + 1, y_idx + 1))
    }

    v
}

fn is_visible(from_x: i32, from_y: i32, to_x: i32, to_y: i32, visible: &[Coords<f64>]) -> Option<AngleCoords> {

    let tmp = (to_x - from_x, to_y - from_y);
    let line_of_sight = normalize_2d(tmp);

    for to_obstacle in visible {
        if dot_product_2d(line_of_sight.0, line_of_sight.1, to_obstacle.0, to_obstacle.1) >= PREC {
            return None
        }
    }

    let dot: f64 = dot_product_2d(line_of_sight.0, line_of_sight.1, 0f64, 1f64);
    let det: f64 = vec_product_2d(line_of_sight.0, line_of_sight.1, 0f64, 1f64);
    let angle = det.atan2(dot).to_degrees();

    Some(AngleCoords{angle, coords: line_of_sight, board_coords: (to_x as usize, to_y as usize)})
}

fn enqueue_vicinity(pt: Coords<usize>, cols: usize, rows: usize, touched: &mut HashSet<Coords<usize>>,
                    to_visit: &mut VecDeque<Coords<usize>>) {

    for pt in get_vicinity(pt.0, pt.1, cols, rows) {
        if touched.contains(&pt) {
            continue
        }

        to_visit.push_back(pt);
        touched.insert(pt);
    }

}

fn find_visible(x: usize, y: usize, field: &[Vec<Point>]) -> Vec<AngleCoords> {

    let mut visible: Vec<Coords<f64>> = Vec::new();
    let mut touched: HashSet<Coords<usize>> = HashSet::new();
    let mut to_visit: VecDeque<Coords<usize>> = VecDeque::new();

    let mut visible_board_coords: Vec<AngleCoords> = Vec::new();

    let cols = field[0].len();
    let rows = field.len();

    let cur_coords = (x, y);

    // add start point into touched list
    touched.insert(cur_coords);

    enqueue_vicinity(cur_coords, cols, rows, &mut touched, &mut to_visit);

    // BFS
    //
    // We start scan at a center point (y_idx, x_idx) and send circle waves with the
    // increasing radius R, so the nearest asteroids will be encountered before farthest ones
    // and we can be sure that new asteroids may only be overlapped by asteroids we already
    // know to be visible.
    //
    while !to_visit.is_empty() {

        let pt = to_visit.pop_front().unwrap();

        touched.insert(pt);

        if field[pt.1][pt.0] == Point::Asteroid {

            let v = is_visible(x as i32, y as i32, pt.0 as i32, pt.1 as i32, &visible);

            if let Some(v) = v {
                visible.push(v.coords);
                visible_board_coords.push(v);
            }

        }

        enqueue_vicinity(pt, cols, rows, &mut touched, &mut to_visit);

    }

    visible_board_coords

}

fn first_star(field: &[Vec<Point>]) -> ProblemResult<(usize, Coords<usize>)> {

    let mut max_visible = 0;
    let mut station_coords = (0, 0);

    for (y_idx, row) in field.iter().enumerate() {

        for (x_idx, _) in row.iter().enumerate() {

            if field[y_idx][x_idx] != Point::Asteroid {
                continue
            }

            let visible_num = find_visible(x_idx, y_idx, field).len();

            if visible_num > max_visible {
                max_visible = visible_num;
                station_coords = (x_idx, y_idx);
            }

        }

    }

    Ok((max_visible, station_coords))

}

fn second_start(field: &mut Vec<Vec<Point>>, base_coords: Coords<usize>) -> ProblemResult<usize> {

    let mut destroyed = 0;

    loop {

        let mut visible = find_visible(base_coords.0, base_coords.1, field);

        visible.sort_unstable_by(|a, b|
            a.angle.partial_cmp(&b.angle).unwrap()
        );

        visible.reverse();

        for p in visible {

            field[p.board_coords.1][p.board_coords.0] = Point::Empty;

            destroyed += 1;

            if destroyed >= TO_DESTROY {
                return Ok(p.board_coords.0 * 100 + p.board_coords.1)
            }

        }

    }

}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");
    let mut input = split_by_lines(
        input_raw, &|line: &str| {
            Ok(split_by(line, "", &|e: &str| { Ok(if e == "." { Point::Empty }
            else { Point::Asteroid }) })?)
        },
    )?;

    let (max_visible, station_coords) = first_star(&input)?;
    Ok(result(Ok(max_visible), second_start(&mut input, station_coords)))
}
