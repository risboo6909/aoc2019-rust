use std::collections::VecDeque;

pub(crate) struct Stdin<T> {
    buf: VecDeque<T>
}

impl<T> Stdin<T> {

    pub fn push(&mut self, item: T) {
        self.buf.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buf.pop_front()
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn new() -> Self {
        return Self {
            buf: VecDeque::new(),
        }
    }

}
