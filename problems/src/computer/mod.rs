use failure::{Error, format_err};

const ADD: isize = 1;
const MUL: isize = 2;
const INP: isize = 3;
const PUT: isize = 4;
const BRK: isize = 99;

fn get_inderect(input: &[isize], idx: usize) -> isize {
    input[input[idx] as usize]
}

fn get_ops(input: &[isize], op_idx: usize) -> (isize, isize, usize) {
    (get_inderect(input, op_idx + 1),
     get_inderect(input, op_idx + 2),
     input[op_idx + 3] as usize)
}

pub(crate) struct Computer {
    memory: isize,
}

impl Computer {

    pub(crate) fn interpret(&mut self, input: &mut [isize]) -> Result<isize, Error> {
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
        Self { memory: 0 }
    }

}
