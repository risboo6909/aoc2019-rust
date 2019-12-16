use std::collections::HashMap;
use rand::prelude::*;

use failure::Error;

use crate::computer::{Computer, parse_intcode};
use utils::{result, ProblemResult, Ret};

#[derive(Hash, Eq, PartialEq)]
struct Coords {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl From<isize> for Color {
    fn from(color: isize) -> Self {
        if color == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl From<Color> for isize {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

impl From<isize> for Turn {
    fn from(dir: isize) -> Self {
        if dir == 0 {
            Turn::Left
        } else {
            Turn::Right
        }
    }
}

enum Direct {
    Up,
    Right,
    Down,
    Left,
}

impl From<isize> for Direct {
    fn from(dir: isize) -> Self {
        if dir == 0 {
            Direct::Up
        } else if dir == 1 {
            Direct::Right
        } else if dir == 2 {
            Direct::Down
        } else {
            Direct::Left
        }
    }
}

fn drawer(program: &[isize], board: &mut HashMap<Coords, Color>) -> ProblemResult<isize> {

    let mut x = 0;
    let mut y = 0;

    let mut direct = 0;

    let mut c = Computer::new(program, Vec::new());

    loop {

        c.push_stdin(isize::from(*board.get(&Coords{x, y}).unwrap_or(&Color::from(0))));

        c.step()?;

        if c.is_finished() {
            break
        }

        let new_color: Color = c.get_output()?.into();
        board.insert(Coords{x, y}, new_color);

        c.step()?;

        let turn: Turn = c.get_output()?.into();

        // assume following:
        // ------------------
        // Up = 0
        // Right = 1
        // Down = 2
        // Left = 3

        direct += match turn {
            Turn::Right => 1,
            Turn::Left => -1,
        };

        if direct < 0 {
            direct = 3;
        }

        direct %= 4;

        match Direct::from(direct) {
            Direct::Up => y -= 1,
            Direct::Down => y += 1,
            Direct::Left => x -= 1,
            Direct::Right => x += 1,
        }

    }

    Ok(0)

}

fn first_star(program: &[isize]) -> ProblemResult<usize> {
    // :-)
    let mut board = HashMap::new();
    drawer(program, &mut board)?;

    Ok(board.len())
}

fn second_star(program: &[isize]) -> ProblemResult<String> {

    let mut board = HashMap::new();
    board.insert(Coords{x: 0, y: 0}, Color::White);

    drawer(program, &mut board)?;

    let min_x = board.keys().fold(std::isize::MAX, |min, e|
        if min > e.x { e.x } else { min }
    );

    let min_y = board.keys().fold(std::isize::MAX, |min, e|
        if min > e.y { e.y } else { min }
    );

    let max_x = board.keys().fold(std::isize::MIN, |max, e|
        if max < e.x { e.x } else { max }
    );

    let max_y = board.keys().fold(std::isize::MIN, |max, e|
        if max < e.y { e.y } else { max }
    );

    let width = max_x - min_x;
    let height = max_y - min_y + 1;

    // translate all points to positive ones
    let mut buf: HashMap<Coords, Color> = HashMap::new();
    for (coords, color) in board.iter() {
        buf.insert(Coords{x: coords.x - min_x, y: coords.y - min_y}, *color);
    }

    let filename = "day11-2.png";

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    let mut rng = rand::thread_rng();

    for (idx, pixel) in imgbuf.pixels_mut().enumerate() {

        match board.get(&Coords {
            x: ((idx as isize) % width),
            y: ((idx as isize) / width),
        }).unwrap_or(&Color::Black) {
            Color::Black => *pixel = image::Rgb([0, 0, 0]),
            Color::White => *pixel = image::Rgb([
                (rng.gen::<f32>() * 255.0) as u8,
                (rng.gen::<f32>() * 255.0) as u8,
                (rng.gen::<f32>() * 255.0) as u8],
            ),
        };

    }

    imgbuf.save(filename).unwrap();

    Ok(filename.to_owned())
}

pub(crate) fn solve() -> Result<Ret<usize, String>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    Ok(result(first_star(&input.clone()), second_star(&input.clone())))
}
