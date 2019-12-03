use crate::{Extract, ProblemInput};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ICFinalization {
    Continue,
    Terminate,
}

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

pub struct ICInstruction {
    /// How many parameters this instruction accepts
    parameters: usize,

    /// A function for evaluating the given instruction
    evaluate: Box<dyn Fn(&mut ICState, Vec<i64>) -> ICFinalization>,
}

pub struct ICInterpreter {
    /// The initial state of the interpreter
    initial_state: ICState,

    /// The current state of our interpreter
    pub state: ICState,

    /// A map indicating which instruction corresponds to a given number
    instructions: HashMap<i64, ICInstruction>,
}

impl ICInterpreter {
    pub fn new(memory: Vec<i64>) -> Self {
        let mut instructions = HashMap::new();

        // Add instruction
        let add_inst = ICInstruction {
            parameters: 3,
            evaluate: Box::new(|state, args| {
                assert_eq!(args.len(), 3); // expect exactly 3 arguments

                // we want to store s + t in state.memory[u]
                let s = state.memory[args[0] as usize];
                let t = state.memory[args[1] as usize];

                state.memory[args[2] as usize] = s + t;

                ICFinalization::Continue
            }),
        };

        instructions.insert(1, add_inst);

        // Mul instruction
        let mul_inst = ICInstruction {
            parameters: 3,
            evaluate: Box::new(|state, args| {
                assert_eq!(args.len(), 3); // expect exactly 3 arguments

                // we want to store s + t in state.memory[u]
                let s = state.memory[args[0] as usize];
                let t = state.memory[args[1] as usize];

                state.memory[args[2] as usize] = s * t;

                ICFinalization::Continue
            }),
        };

        instructions.insert(2, mul_inst);

        // Terminate instruction
        let term_inst = ICInstruction {
            parameters: 0,
            evaluate: Box::new(|_, _| ICFinalization::Terminate),
        };

        instructions.insert(99, term_inst);

        Self {
            initial_state: ICState::new(memory.clone()),
            state: ICState::new(memory),
            instructions,
        }
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state.clone();
    }

    pub fn register(&mut self, key: i64, inst: ICInstruction) {
        self.instructions.insert(key, inst);
    }

    pub fn run(&mut self) -> ICState {
        loop {
            // get the current instruction key
            let key = self.state.get_current_state();
            let inst = self.instructions.get_mut(&key).unwrap();

            // collect the arguments
            let args = self.state.get_parameters(inst.parameters);

            // evaluate the instruction
            let result = (inst.evaluate)(&mut self.state, args);

            match result {
                ICFinalization::Continue => {
                    self.state.jump_by(inst.parameters + 1);
                }
                ICFinalization::Terminate => {
                    break;
                }
            }
        }

        self.state.clone()
    }
}

impl Clone for ICInterpreter {
    fn clone(&self) -> Self {
        ICInterpreter::new(self.state.memory.clone())
    }
}

impl Extract<ICInterpreter> for ProblemInput {
    fn extract(&self) -> Result<ICInterpreter> {
        // this will form our memory
        let inner: Vec<i64> = self.extract()?;

        Ok(ICInterpreter::new(inner))
    }
}
