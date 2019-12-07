use failure::{Error, format_err};
use permutohedron as ph;

use crate::computer::Computer;
use utils::{split_by_comma, result, ProblemResult, RetOne};


fn first_star(program: &[isize]) -> ProblemResult<isize> {

    let mut xs: [isize; 5] = [0, 1, 2, 3, 4];
    let perms = ph::Heap::new(&mut xs);

    let mut best_val = 0;

    for perm in perms {

        let mut c = Computer::new(program, vec![perm[0], 0]);
        c.interpret()?;

        for j in perm.iter().skip(1) {
            c = Computer::new(program, vec![*j, c.get_output()?]);
            c.interpret()?;
        }

        let final_val = c.get_output()?;
        if final_val > best_val {
            best_val = final_val;
        }
    }

    Ok(best_val)
}

fn exec_amp(amp: &mut Computer, input: isize) -> ProblemResult<(bool, isize)> {

    amp.push_stdin(input);
    amp.interpret()?;

    if amp.is_finished() {
        return Ok((true, 0));
    }

    Ok((false, amp.get_output().unwrap()))

}

fn second_star(program: &[isize]) -> ProblemResult<isize> {

    let mut xs: [isize; 5] = [5, 6, 7, 8, 9];
    let mut best_val = 0;

    let perms = ph::Heap::new(&mut xs);

    for perm in perms {

        let mut input = 0;
        let mut amps = Vec::new();

        for j in perm.iter() {
            amps.push(Computer::new(program, vec![*j]))
        }

        'outer:
        loop {

            for amp in amps.iter_mut().take(4 + 1) {

                let r = exec_amp(amp, input)?;
                if r.0 {
                    break 'outer;
                }

                input = r.1;
            }

            for amp in amps.iter_mut().take(4 + 1) {
                amp.clear_stdin();
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
    let input = split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))?;

    //first_star(&input.clone())
    Ok(result(first_star(&input.clone()), second_star(&input.clone())))
}
