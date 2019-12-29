use std::collections::{HashMap, HashSet, VecDeque};
use failure::Error;

use utils::{result, ProblemResult, RetOne};
use crate::computer::{Computer, parse_intcode};

#[derive(Copy, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

type Coords = (isize, isize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Output {
    Wall,
    Moved,
    Oxygen,
}

impl From<isize> for Output {
    fn from(i: isize) -> Self {
        match i {
            0 => Output::Wall,
            1 => Output::Moved,
            2 => Output::Oxygen,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    moves_cnt: usize,
    tile: Output,
}

impl Dir {

    fn to_coords(self, cur_pos: Coords) -> Coords {
        match self {
            Dir::North => (cur_pos.0, cur_pos.1 - 1),
            Dir::South => (cur_pos.0, cur_pos.1 + 1),
            Dir::West => (cur_pos.0 - 1, cur_pos.1),
            Dir::East => (cur_pos.0 + 1, cur_pos.1),
        }
    }

    fn opposite_dir(self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }

}

impl From<Dir> for isize {
    fn from(d: Dir) -> Self {
        match d {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

type Area = HashMap<Coords, Tile>;


fn make_move(c: &mut Computer, move_to: Dir) -> ProblemResult<Output> {

    c.set_stdin(move_to.into());

    Ok(
        loop {

            // wait until droid finishes its movement and get its output

            c.step()?;

            match c.get_output() {
                Ok(x) => break x,
                Err(_) => {
                    // no output yet, droid is still moving
                    continue;
                }
            }

        }.into()
    )

}

fn rec(c: &mut Computer, program: &[isize], cur_pos: Coords, visited: &mut Area, moves_cnt: usize) -> ProblemResult<usize> {

    let mut min_score = std::usize::MAX;

    for dir in DIRS.iter() {

        let new_coords = dir.to_coords(cur_pos);
        if visited.contains_key(&new_coords) && visited[&new_coords].moves_cnt <= (moves_cnt + 1) {
            continue
        }

        // move forward
        let tile = make_move(c, *dir)?;

        visited.insert(new_coords, Tile{moves_cnt, tile});

        if tile == Output::Wall {
            // drone position isn't changed if it hits a wall
            continue
        }

        if tile == Output::Oxygen {
            min_score = moves_cnt + 1;
        }

        let score = rec(c, program, new_coords, visited, moves_cnt + 1)?;
        if score < min_score {
            min_score = score;
        }

        // rewind back
        make_move(c, dir.opposite_dir())?;

    }

    Ok(min_score)

}

fn first_star(program: &[isize]) -> ProblemResult<(usize, Area)> {

    let mut c = Computer::new(program, None);
    let mut visited: Area = HashMap::new();

    c.step()?;
    let res = rec(&mut c, &program, (0, 0), &mut visited, 0)?;

    Ok((res, visited))

}

fn get_vicinity(map: &Area, coords: Coords) -> VecDeque<Coords> {

    let mut tmp = VecDeque::new();

    for inc in &[1, -1] {
        if let Some(tile) = map.get(&(coords.0 + inc, coords.1)) {
            if tile.tile != Output::Wall {
                tmp.push_back((coords.0 + inc, coords.1))
            }
        }
    }

    for inc in &[1, -1] {
        if let Some(tile) = map.get(&(coords.0, coords.1 + inc)) {
            if tile.tile != Output::Wall {
                tmp.push_back((coords.0, coords.1 + inc))
            }
        }
    }

    tmp

}

fn second_star(map: &Area) -> ProblemResult<usize> {

    let mut oxygen_coords = Coords::default();

    for (coords, tile) in map {
        if tile.tile == Output::Oxygen {
            oxygen_coords = *coords;
            break
        }
    };

    let mut q = get_vicinity(map, oxygen_coords);
    let mut visited: HashSet<Coords> = HashSet::new();

    visited.insert(oxygen_coords);

    let mut iters = 0;

    loop {

        for coords in q.drain(..).collect::<HashSet<Coords>>() {
            if visited.contains(&coords) {
                continue
            };

            visited.insert(coords);
            q.extend(get_vicinity(map, coords));
        }

        if q.is_empty() {
            break
        }

        iters += 1;

    }

    Ok(iters)
}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    let (r1, visited) = first_star(&input.clone())?;
    let r2 = second_star(&visited);

    assert_eq!(r1, 282);
    assert_eq!(*r2.as_ref().unwrap(), 286);

    Ok(result(Ok(r1), r2))
}
