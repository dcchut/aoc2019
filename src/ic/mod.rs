use crate::ic::state::ICState;

pub mod interpreter;
pub mod io;
pub mod orchestrator;
pub mod state;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ICPostAction {
    Continue,
    NoMove,
    Terminate,
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct ICCode {
    /// The value of this code when used as a write index
    pub index: usize,

    /// The value of this code when used as a value
    pub value: i64,

    /// The parameter mode of this code
    mode: ICMode,
}

impl ICCode {
    pub fn new(state: &ICState, value: i64, mode: ICMode) -> Self {
        let index = match mode {
            ICMode::Relative => state.relative_base + value,
            _ => value,
        } as usize;

        let value = match mode {
            ICMode::Immediate => value,
            _ => state.read(index),
        };

        Self { index, value, mode }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ICMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ICMode {
    fn from(x: i64) -> Self {
        match x {
            1 => ICMode::Immediate,
            2 => ICMode::Relative,
            0 => ICMode::Position,
            _ => panic!("Invalid interpreter mode detected"),
        }
    }
}
