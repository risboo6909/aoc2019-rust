mod op;
mod helpers;

use std::collections::{HashMap, VecDeque};

use failure::{Error, format_err};
use utils::{split_digits, ParseResult};
use op::{Modes, Mode, Op, Operands, Arg};

pub(crate) use helpers::consume_until_break;

const ADD: isize = 1;
const MUL: isize = 2;
const INP: isize = 3;
const PUT: isize = 4;
const JMPT: isize = 5;
const JMPF: isize = 6;
const LT: isize = 7;
const EQ: isize = 8;
const BASE: isize = 9;
const BRK: isize = 99;


pub(crate) struct Computer {
    pub stdout: Option<isize>,
    pub stdin: VecDeque<isize>,
    pub finished: bool,

    program: HashMap<usize, isize>,
    offset: isize,
    ip: usize,
}

impl Computer {

    pub(crate) fn new(input_program: &[isize], input: Vec<isize>) -> Self {

        let mut program: HashMap<usize, isize> = HashMap::new();
        let mut tmp = VecDeque::new();

        tmp.extend(input);

        // copy program
        for (idx, item) in input_program.iter().enumerate() {
            program.insert(idx, *item);
        }

        Self {
            stdout: None,
            stdin: tmp,
            finished: false,

            program,
            offset: 0,
            ip: 0,
        }

    }

    fn set_cell(&mut self, idx: usize, val: isize) {
        self.program.insert(idx as usize, val);
    }

    pub(crate) fn get_cell(&self, idx: usize) -> isize {
        *self.program.get(&(idx as usize)).unwrap_or(&0)
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

            let op = Self::parse_op(self.get_cell(self.ip));

            match op.op_code {

                ADD => {
                    if let Operands::Three(a, b, to) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(self.get_arg_addr(to)?, self.get_arg_value(a)? + self.get_arg_value(b)?);
                        self.ip += 4;
                    }
                },

                MUL => {
                    if let Operands::Three(a, b , to) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(self.get_arg_addr(to)?, self.get_arg_value(a)? * self.get_arg_value(b)?);
                        self.ip += 4;
                    }
                },

                INP => {
                    if let Operands::One(a) = self.get_ops(self.ip, &op.mode_flags, 1)? {
                        let v = self.stdin.pop_front().unwrap();
                        self.set_cell(self.get_arg_addr(a)? as usize, v);
                        self.ip += 2;
                    }
                },

                PUT => {
                    if let Operands::One(a) = self.get_ops(self.ip, &op.mode_flags, 1)? {
                        self.stdout = Some(self.get_arg_value(a)?);
                        self.ip += 2;
                        break;
                    }
                },

                JMPT => {
                    if let Operands::Two(value, to) = self.get_ops(self.ip, &op.mode_flags, 2)? {
                        if self.get_arg_value(value)? != 0 {
                            self.ip = self.get_arg_value(to)? as usize;
                        } else {
                            self.ip += 3;
                        }
                    }
                },

                JMPF => {
                    if let Operands::Two(value, to) = self.get_ops(self.ip, &op.mode_flags, 2)? {
                        if self.get_arg_value(value)? == 0 {
                            self.ip = self.get_arg_value(to)? as usize;
                        } else {
                            self.ip += 3;
                        }
                    }
                },

                LT => {
                    if let Operands::Three(a, b, to) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(self.get_arg_addr(to)? as usize, if self.get_arg_value(a)? < self.get_arg_value(b)? {
                            1
                        } else {
                            0
                        });
                        self.ip += 4;
                    }
                },

                EQ => {
                    if let Operands::Three(a, b, to) = self.get_ops(self.ip, &op.mode_flags, 3)? {
                        self.set_cell(self.get_arg_addr(to)? as usize, if self.get_arg_value(a)? == self.get_arg_value(b)? {
                            1
                        } else {
                            0
                        });
                        self.ip += 4;
                    }
                },

                BASE => {
                    //op.mode_flags.mark_direct(0);
                    if let Operands::One(a) = self.get_ops(self.ip, &op.mode_flags, 1)? {
                        self.offset += self.get_arg_value(a)?;
                        self.ip += 2;
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

    fn get_arg_value(&self, arg: Arg) -> ParseResult<isize> {
        Ok(match arg.mode {
            Mode::Direct => arg.value,
            _ => self.get_cell(arg.value as usize),
        })
    }

    fn get_arg_addr(&self, arg: Arg) -> ParseResult<usize> {
        match arg.mode {
            Mode::Direct => Err(format_err!("Address can't be in direct mode")),
            _ => Ok(arg.value as usize),
        }
    }

    fn get_operand(&self, value: isize, mode: op::Mode) -> Arg {
        match mode {
            op::Mode::Direct => Arg{value, mode},
            op::Mode::Relative => Arg{value: self.offset + value, mode},
            op::Mode::Indirect => Arg{value, mode},
        }
    }

    fn get_ops(&self, op_idx: usize, mode_flags: &Modes, args_num: isize) -> ParseResult<Operands> {

        match args_num {

            1 => Ok(
                Operands::One(
                    self.get_operand(self.get_cell(op_idx + 1), mode_flags.get_mode(0)),
                )
            ),

            2 => Ok(
                Operands::Two(
                    self.get_operand(self.get_cell(op_idx + 1), mode_flags.get_mode(0)),
                    self.get_operand(self.get_cell(op_idx + 2), mode_flags.get_mode(1)),
                )
            ),

            3 => Ok(
                Operands::Three(
                    self.get_operand(self.get_cell(op_idx + 1), mode_flags.get_mode(0)),
                    self.get_operand(self.get_cell(op_idx + 2), mode_flags.get_mode(1)),
                    self.get_operand(self.get_cell(op_idx + 3), mode_flags.get_mode(2)),
                )
            ),

            n => Err(format_err!("Wrong number of arguments {}", n)),

        }

    }

}
