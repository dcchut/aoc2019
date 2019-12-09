use crate::ic::interpreter::ICInterpreter;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ICState {
    /// The memory store of our IC interpreter
    pub memory: Vec<i64>,

    /// The current instruction pointer
    pub ip: usize,

    /// The current relative base
    pub relative_base: i64,
}

impl ICState {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            ip: 0,
            relative_base: 0,
        }
    }

    #[inline(always)]
    pub fn get_current_state(&self) -> i64 {
        self.read(self.ip)
    }

    #[inline(always)]
    pub fn get_parameters(&self, parameters: usize) -> Vec<i64> {
        (&self.memory[(self.ip + 1)..=(self.ip + parameters)]).to_vec()
    }

    #[inline(always)]
    pub fn jump_by(&mut self, jump_by: usize) {
        self.ip += jump_by;
    }

    #[inline(always)]
    pub fn read(&self, index: usize) -> i64 {
        if index >= self.memory.len() {
            0
        } else {
            self.memory[index]
        }
    }

    #[inline(always)]
    pub fn write(&mut self, index: usize, value: i64) {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }
}

#[derive(Debug, Clone)]
pub struct ICTerminalState<'a> {
    state: &'a ICState,
    pub opcode: usize,
}

impl<'a> ICTerminalState<'a> {
    pub fn new(interpreter: &'a ICInterpreter) -> Self {
        Self {
            state: &interpreter.state,
            opcode: interpreter.opcode,
        }
    }
}

impl Deref for ICTerminalState<'_> {
    type Target = ICState;

    fn deref(&self) -> &Self::Target {
        self.state
    }
}
