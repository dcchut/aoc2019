use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct ICInput {
    pub buffer: VecDeque<i64>,
}

impl ICInput {
    pub fn single(single: i64) -> Self {
        let mut buffer = VecDeque::new();
        buffer.push_front(single);

        Self { buffer }
    }

    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }
}

impl Default for ICInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<i64>> for ICInput {
    fn from(buffer: Vec<i64>) -> Self {
        Self {
            buffer: buffer.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ICOutput {
    pub outputs: VecDeque<i64>,
}

impl ICOutput {
    pub fn new() -> Self {
        Self {
            outputs: VecDeque::new(),
        }
    }

    pub fn last(&self) -> Option<i64> {
        self.outputs.back().cloned()
    }
}

impl Default for ICOutput {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Queue<T> {
    fn add(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn reset(&mut self);
}

// Internal trait for something that wraps a VecDeque.
pub trait QueueWrapper<T> {
    fn as_queue(&mut self) -> &mut VecDeque<T>;
}

impl QueueWrapper<i64> for ICInput {
    fn as_queue(&mut self) -> &mut VecDeque<i64> {
        &mut self.buffer
    }
}

impl QueueWrapper<i64> for ICOutput {
    fn as_queue(&mut self) -> &mut VecDeque<i64> {
        &mut self.outputs
    }
}

impl<S, T> Queue<S> for T
where
    T: QueueWrapper<S>,
{
    fn add(&mut self, val: S) {
        self.as_queue().push_back(val);
    }

    fn pop(&mut self) -> Option<S> {
        self.as_queue().pop_front()
    }

    fn reset(&mut self) {
        self.as_queue().clear();
    }
}
