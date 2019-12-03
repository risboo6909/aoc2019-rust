use failure::{Error, format_err};
use utils::{split_by_comma, result, ProblemResult, Ret};

const ADD: u32 = 1;
const MUL: u32 = 2;
const BRK: u32 = 99;

fn get_inderect(input: &[u32], idx: usize) -> u32 {
    input[input[idx] as usize]
}

fn get_ops(input: &[u32], op_idx: usize) -> (u32, u32, usize) {
    (get_inderect(input, op_idx + 1),
     get_inderect(input, op_idx + 2),
     input[op_idx + 3] as usize)
}

fn interpret(input: &mut [u32]) -> Result<u32, Error> {
    let mut idx = 0;
    loop {
        match input[idx] {
            ADD => {
                let (a, b, to_idx) = get_ops(input, idx);
                input[to_idx] = a + b;
            },
            MUL => {
                let (a, b, to_idx) = get_ops(input, idx);
                input[to_idx] = a * b;
            },
            BRK => {
                return Ok(input[0]);
            },
            s => { return Err(format_err!("Unknown state {}", s)) },
        }
        idx += 4;
    }
}

fn first_star(program: &mut [u32]) -> ProblemResult<u32> {
    // input for the program
    program[1] = 12;
    program[2] = 2;

    // run the program
    interpret(program)
}

fn second_star(program: &mut [u32]) -> ProblemResult<u32> {

    let mut saved_program: Vec<u32> = vec![0; program.len()];
    saved_program.clone_from_slice(&program);

    for n in 0..10000 {

        let program = &mut saved_program.clone();

        let (noun, verb) = (n % 100u32, n / 100u32);

        // input for the program
        program[1] = noun;
        program[2] = verb;

        if interpret(program)? == 19690720 {
            return Ok(100 * noun + verb);
        }
    }

    Err(format_err!("Couldn't find appropriate solution!"))

}

pub(crate) fn solve() -> Result<Ret<u32>, Error> {
    let input_raw = include_str!("./input");
    let mut input: Vec<u32> = split_by_comma(input_raw, &|e: &str| e.parse::<u32>()
        .or(Err(format_err!("Failed to parse input"))))?;

    Ok(result(first_star(&mut input.clone()), second_star(&mut input.clone())))
}
