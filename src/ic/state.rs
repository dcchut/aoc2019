use crate::ic::interpreter::ICInterpreter;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ICState {
    /// The memory store of our IC interpreter
    pub memory: Vec<i64>,

    /// The current instruction pointer
    pub ip: usize,
}

impl ICState {
    pub fn new(memory: Vec<i64>) -> Self {
        Self { memory, ip: 0 }
    }

    #[inline(always)]
    pub fn get_state(&self, index: usize) -> i64 {
        self.memory[index]
    }

    #[inline(always)]
    pub fn get_current_state(&self) -> i64 {
        self.get_state(self.ip)
    }

    #[inline(always)]
    pub fn get_parameters(&self, parameters: usize) -> Vec<i64> {
        (&self.memory[(self.ip + 1)..=(self.ip + parameters)]).to_vec()
    }

    #[inline(always)]
    pub fn jump_by(&mut self, jump_by: usize) {
        self.ip += jump_by;
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
