mod stdin;

use failure::{Error, format_err};

pub(crate) use stdin::Stdin;

const ADD: u32 = 1;
const MUL: u32 = 2;
const INP: u32 = 3;
const PUT: u32 = 4;
const BRK: u32 = 99;

fn get_inderect(input: &[u32], idx: usize) -> u32 {
    input[input[idx] as usize]
}

fn get_ops(input: &[u32], op_idx: usize) -> (u32, u32, usize) {
    (get_inderect(input, op_idx + 1),
     get_inderect(input, op_idx + 2),
     input[op_idx + 3] as usize)
}

pub(crate) struct Computer<T> {
    stdin: Stdin<T>,
}

impl<T> Computer<T> {

    pub(crate) fn interpret(&mut self, input: &mut [u32]) -> Result<u32, Error> {
        let mut idx = 0;
        loop {
            match input[idx] {
                ADD => {
                    let (a, b, to_idx) = get_ops(input, idx);
                    input[to_idx] = a + b;
                    idx += 4;
                },
                MUL => {
                    let (a, b, to_idx) = get_ops(input, idx);
                    input[to_idx] = a * b;
                    idx += 4;
                },
                INP => {
                    match self.stdin.pop() {
                        Some(v) => v,
                        None => return Err(format_err!("Attempt to read from empty buffer")),
                    };
                    idx += 2;
                },
                PUT => {

                },
                BRK => {
                    return Ok(input[0]);
                },
                s => { return Err(format_err!("Unknown state {}", s)) },
            }
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            stdin: Stdin::new(),
        }
    }

}
