use failure::Error;
use permutohedron as ph;

use crate::computer::{parse_intcode, Computer};
use utils::{result, ProblemResult, RetOne};

const AMPLIFIERS: isize = 5;

fn first_star(program: &[isize]) -> ProblemResult<isize> {
    let xs = &mut (0..AMPLIFIERS).collect::<Vec<isize>>();
    let perms = ph::Heap::new(xs);

    let mut best_val = 0;

    for perm in perms {
        let mut c = Computer::new(program, Some(vec![perm[0], 0]));
        c.step()?;

        for j in perm.iter().skip(1) {
            c = Computer::new(program, Some(vec![*j, c.get_output()?]));
            c.step()?;
        }

        let final_val = c.get_output()?;
        if final_val > best_val {
            best_val = final_val;
        }
    }

    Ok(best_val)
}

fn exec_amp(amp: &mut Computer, input: isize) -> ProblemResult<(bool, isize)> {
    amp.step()?;

    if amp.is_finished() {
        return Ok((true, 0));
    } else if amp.waits_input() {
        amp.set_stdin(input);
        amp.step()?;
    }

    Ok((false, amp.get_output().unwrap()))
}

fn second_star(program: &[isize]) -> ProblemResult<isize> {
    let xs = &mut (AMPLIFIERS..2 * AMPLIFIERS).collect::<Vec<isize>>();
    let perms = ph::Heap::new(xs);

    let mut best_val = 0;

    for perm in perms {
        let mut input = 0;
        let mut amps = Vec::new();

        for j in perm.iter() {
            amps.push(Computer::new(program, Some(vec![*j])))
        }

        'outer: loop {
            for amp in amps.iter_mut() {
                let r = exec_amp(amp, input)?;
                if r.0 {
                    break 'outer;
                }

                input = r.1;
            }
        }

        if input > best_val {
            best_val = input;
        }
    }

    Ok(best_val)
}

pub(crate) fn solve() -> Result<RetOne<isize>, Error> {
    let input_raw = include_str!("./input");
    let input = parse_intcode(input_raw)?;

    let r1 = first_star(&input.clone());
    let r2 = second_star(&input.clone());

    assert_eq!(*r1.as_ref().unwrap(), 38_500);
    assert_eq!(*r2.as_ref().unwrap(), 33_660_560);

    Ok(result(r1, r2))
}
