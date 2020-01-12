use failure::Error;

use crate::computer::{parse_intcode, stop_or_input, Computer};
use utils::{result, ProblemResult, RetOne};

const BLOCK: usize = 2;

const STAY: isize = 0;
const LEFT: isize = -1;
const RIGHT: isize = 1;

struct AI {
    pad_x: isize,
}

impl AI {
    pub(crate) fn get_move_dir(&mut self, ball_x: isize) -> isize {
        // move the pad
        if ball_x > self.pad_x {
            self.pad_x += 1;
            RIGHT
        } else if ball_x < self.pad_x {
            self.pad_x -= 1;
            LEFT
        } else {
            STAY
        }
    }
}

fn first_star(program: &[isize]) -> ProblemResult<usize> {
    let mut blocks_count = 0;
    let mut c = Computer::new(program, None);

    Ok(loop {
        c.step()?;
        if c.is_finished() {
            break blocks_count;
        }

        c.get_output().unwrap();

        c.step()?;
        c.get_output().unwrap();

        c.step()?;
        let title_id = c.get_output().unwrap() as usize;

        if title_id == BLOCK {
            blocks_count += 1;
        }
    })
}

fn second_star(program: &mut [isize]) -> ProblemResult<usize> {
    // setup initial state
    program[0] = 2;

    let mut c = Computer::new(program, None);

    let mut x = 0;
    let mut score = 0;

    let mut ai = AI { pad_x: 22 };

    Ok(loop {
        c.step()?;

        if stop_or_input(&mut c, || ai.get_move_dir(x))? {
            break score;
        }
        x = c.get_output()?;

        c.step()?;
        let y = c.get_output()?;

        c.step()?;
        if x == -1 && y == 0 {
            score = c.get_output()? as usize;
        }
    })
}

pub(crate) fn solve() -> Result<RetOne<usize>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    let r1 = first_star(&input.clone());
    let r2 = second_star(&mut input.clone());

    assert_eq!(*r1.as_ref().unwrap(), 326);
    assert_eq!(*r2.as_ref().unwrap(), 15_988);

    Ok(result(r1, r2))
}
