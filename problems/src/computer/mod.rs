mod op;

use std::collections::VecDeque;

use failure::{Error, format_err};
use utils::{split_digits, ParseResult};
use op::{Modes, Op, Operands};

const ADD: isize = 1;
const MUL: isize = 2;
const INP: isize = 3;
const PUT: isize = 4;
const JMPT: isize = 5;
const JMPF: isize = 6;
const LT: isize = 7;
const EQ: isize = 8;
const BRK: isize = 99;


pub(crate) struct Computer {
    program: Vec<isize>,
    pub stdout: Option<isize>,
    pub stdin: VecDeque<isize>,

    pub finished: bool,
    ip: usize,
}

impl Computer {

    pub(crate) fn new(program: &[isize], input: Vec<isize>) -> Self {

        let mut tmp = VecDeque::new();

        tmp.extend(input);

        Self {
            program: program.to_owned(),
            stdout: None,
            stdin: tmp,

            finished: false,
            ip: 0,
        }

    }

    fn set_cell(&mut self, idx: isize, val: isize) {
        self.program[idx as usize] = val;
    }

    pub(crate) fn get_cell(&mut self, idx: isize) -> isize {
        self.program[idx as usize]
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished
    }

    pub(crate) fn get_output(&mut self) -> Result<isize, Error> {

        // error here helps find subtle bugs

        match self.stdout {

            Some(x) => {
                self.stdout = None;
                Ok(x)
            },

            None => Err(format_err!("Output exhausted")),
        }

    }

    pub(crate) fn push_stdin(&mut self, val: isize) {
        self.stdin.push_back(val);
    }

    pub(crate) fn clear_stdin(&mut self) {
        self.stdin.clear()
    }

    pub(crate) fn interpret(&mut self) -> Result<isize, Error> {

        loop {

            let mut op = Self::parse_op(self.program[self.ip]);

            match op.op_code {

                ADD => {
                    if let Operands::Three(a, b, to_idx) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(to_idx, a + b);
                        self.ip += 4;
                    }
                },

                MUL => {
                    if let Operands::Three(a, b , to_idx) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(to_idx, a * b);
                        self.ip += 4;
                    }
                },

                INP => {
                    op.mode_flags.mark_direct(0);
                    if let Operands::One(to_idx) = self.get_ops(self.ip, &op.mode_flags, 1)? {
                        let v = self.stdin.pop_front().unwrap();
                        self.set_cell(to_idx, v);
                        self.ip += 2;
                    }
                },

                PUT => {
                    if let Operands::One(value) = self.get_ops(self.ip, &op.mode_flags, 1)? {
                        self.stdout = Some(value);
                        self.ip += 2;
                        break;
                    }
                },

                JMPT => {
                    if let Operands::Two(value, to_idx) = self.get_ops(self.ip, &op.mode_flags, 2)? {
                        if value != 0 {
                            self.ip = to_idx as usize;
                        } else {
                            self.ip += 3;
                        }
                    }
                },

                JMPF => {
                    if let Operands::Two(value, to_idx) = self.get_ops(self.ip, &op.mode_flags, 2)? {
                        if value == 0 {
                            self.ip = to_idx as usize;
                        } else {
                            self.ip += 3;
                        }
                    }
                },

                LT => {
                    if let Operands::Three(a, b, to_idx) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(to_idx, if a < b { 1 } else { 0 });
                        self.ip += 4;
                    }
                },

                EQ => {
                    if let Operands::Three(a, b, to_idx) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(to_idx, if a == b { 1 } else { 0 });
                        self.ip += 4;
                    }
                },

                BRK => {
                    self.finished = true;
                    break;
                },

                s => { return Err(format_err!("Unknown state {}", s)) },

            }

        }

        Ok(0)

    }

    fn parse_op(op: isize) -> Op {

        let op_code = op % 100;
        let mode = op / 100;
        let mut mode_flags = split_digits(mode as usize);

        mode_flags.reverse();

        Op {
            op_code,
            mode_flags: Modes::new(&mode_flags),
        }

    }

    fn get_operand(&self, value: isize, mode: op::Mode) -> isize {
        if mode == op::Mode::Direct {
            return value
        }
        self.program[value as usize]
    }

    fn get_ops(&self, op_idx: usize, mode_flags: &Modes, args_num: isize) -> ParseResult<Operands> {

        match args_num {

            1 => Ok(
                Operands::One(
                    self.get_operand(self.program[op_idx + 1], mode_flags.get_mode(0)),
                )
            ),

            2 => Ok(
                Operands::Two(
                    self.get_operand(self.program[op_idx + 1], mode_flags.get_mode(0)),
                    self.get_operand(self.program[op_idx + 2], mode_flags.get_mode(1)),
                )
            ),

            3 => Ok(
                Operands::Three(
                    self.get_operand(self.program[op_idx + 1], mode_flags.get_mode(0)),
                    self.get_operand(self.program[op_idx + 2], mode_flags.get_mode(1)),
                    self.get_operand(self.program[op_idx + 3], op::Mode::Direct),
                )
            ),

            n => Err(format_err!("Wrong number of arguments {}", n)),

        }

    }

}
