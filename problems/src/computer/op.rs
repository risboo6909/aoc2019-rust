use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Mode {
    Indirect,
    Direct,
}

#[derive(Debug)]
pub(crate) struct Modes {
    mode_flags: HashMap<usize, Mode>,
}

impl Modes {

    pub(crate) fn new(mode_flags: &[usize]) -> Self {

        let mut tmp = HashMap::new();

        for (idx, flag) in mode_flags.iter().enumerate() {
            tmp.insert(idx, if *flag == 0 {
                Mode::Indirect
            } else {
                Mode::Direct
            });
        }

        Self {
            mode_flags: tmp
        }

    }

    pub(crate) fn mark_direct(&mut self, pos: usize) {
        self.mode_flags.insert(pos, Mode::Direct);
    }

    pub(crate) fn get_mode(&self, idx: usize) -> Mode {
        if !self.mode_flags.contains_key(&idx) {
            return Mode::Indirect;
        }
        self.mode_flags[&idx]
    }

}

// one operator description
#[derive(Debug)]
pub(crate) struct Op {
    pub op_code: isize,
    pub mode_flags: Modes,
}

// each operator can have one, two or three operands
pub(crate) enum Operands {
    One(isize),
    Two(isize, isize),
    Three(isize, isize, isize),
}
