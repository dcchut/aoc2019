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
    pub immediate: i64,
    pub value : i64,
    mode: ICMode,
}

impl ICCode {
    pub fn new(state: &ICState, value: i64, mode: ICMode) -> Self {
        Self {
            immediate: value,
            value: match mode {
                ICMode::Position => state.memory[value as usize],
                ICMode::Immediate => value,
            },
            mode,
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ICMode {
    Position,
    Immediate,
}

impl From<i64> for ICMode {
    fn from(x: i64) -> Self {
        if x == 1 {
            ICMode::Immediate
        } else {
            ICMode::Position
        }
    }
}
