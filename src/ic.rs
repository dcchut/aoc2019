use crate::{Digits, Extract, FromDigits, ProblemInput};
use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::RwLock;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ICFinalization {
    Continue,
    NoMove,
    Terminate,
}

#[derive(Debug, Clone)]
pub struct ICState {
    /// The memory store of our IC interpreter
    pub memory: Vec<i64>,

    /// The current instruction pointer
    pub ip: usize,

    /// A collection of inputs
    pub inputs: Rc<RwLock<InterpreterInput>>,

    /// A collection of outputs
    pub outputs: Vec<i64>,
}

impl ICState {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            ip: 0,
            outputs: Vec::new(),
            inputs: Rc::new(RwLock::new(InterpreterInput::new())),
        }
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ICMode {
    Position,
    Immediate,
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct ICCode {
    value: i64,
    mode: ICMode,
}

impl ICCode {
    pub fn value(&self, state: &ICState) -> i64 {
        // evaluate the current code at the given state
        match self.mode {
            ICMode::Position => state.get_state(self.value as usize),
            ICMode::Immediate => self.value,
        }
    }
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

#[derive(Debug, Clone)]
pub struct ICTerminalState {
    state: ICState,
    pub last_instruction: usize,
}

impl Deref for ICTerminalState {
    type Target = ICState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

pub struct ICInstruction {
    /// How many parameters this instruction accepts
    parameters: usize,

    /// A function for evaluating the given instruction
    evaluate: Box<dyn Fn(&mut ICState, Vec<ICCode>) -> ICFinalization>,
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
    pub fn register<F>(&mut self, key: i64, parameters: usize, f: F)
    where
        F: 'static + Fn(&mut ICState, Vec<ICCode>) -> ICFinalization,
    {
        // Box our closure up, together with an assertion that it receives the correct number of arguments
        let evaluate = Box::new(move |state: &mut ICState, args: Vec<ICCode>| {
            assert_eq!(args.len(), parameters);

            f(state, args)
        });

        let instruction = ICInstruction {
            parameters,
            evaluate,
        };

        self.instructions.insert(key, instruction);
    }
    pub fn new(memory: Vec<i64>) -> Self {
        let mut interpreter = Self {
            initial_state: ICState::new(memory.clone()),
            state: ICState::new(memory),
            instructions: HashMap::new(),
        };

        // Add instruction
        interpreter.register(1, 3, |state, args| {
            let s = args[0].value(state);
            let t = args[1].value(state);

            state.memory[args[2].value as usize] = s + t;

            ICFinalization::Continue
        });

        // Mul instruction
        interpreter.register(2, 3, |state, args| {
            let s = args[0].value(state);
            let t = args[1].value(state);
            state.memory[args[2].value as usize] = s * t;

            ICFinalization::Continue
        });

        // Terminate instruction
        interpreter.register(99, 0, |_, _| ICFinalization::Terminate);

        // Input instruction
        interpreter.register(3, 1, |state, args| {
            state.memory[args[0].value as usize] = state.inputs.write().unwrap().pop();

            ICFinalization::Continue
        });

        // Output instruction
        interpreter.register(4, 1, |state, args| {
            state.outputs.push(args[0].value(state));

            ICFinalization::Terminate
        });

        // jump-if-true instruction
        interpreter.register(5, 2, |state, args| {
            let u = args[0].value(state);
            let v = args[1].value(state);

            if u != 0 {
                state.ip = v as usize;

                ICFinalization::NoMove
            } else {
                ICFinalization::Continue
            }
        });

        // jump_if_false instruction
        interpreter.register(6, 2, |state, args| {
            let u = args[0].value(state);
            let v = args[1].value(state);

            if u == 0 {
                state.ip = v as usize;

                ICFinalization::NoMove
            } else {
                ICFinalization::Continue
            }
        });

        // lt instruction
        interpreter.register(7, 3, |state, args| {
            let s = args[0].value(state);
            let t = args[1].value(state);

            state.memory[args[2].value as usize] = if s < t { 1 } else { 0 };

            ICFinalization::Continue
        });

        // eq instruction
        interpreter.register(8, 3, |state, args| {
            let s = args[0].value(state);
            let t = args[1].value(state);

            state.memory[args[2].value as usize] = if s == t { 1 } else { 0 };

            ICFinalization::Continue
        });

        interpreter
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state.clone();
    }

    pub fn run_with_inputs(&mut self, inputs: &Rc<RwLock<InterpreterInput>>) -> ICTerminalState {
        // Inject the given inputs into the state
        self.state.inputs = Rc::clone(inputs);
        let mut opcode;

        loop {
            // get the current instruction key
            let key = self.state.get_current_state();

            // process the key into an ICCode
            let mut digits = key.digits();

            // last two digits are the opcode
            opcode = {
                if digits.len() == 1 {
                    vec![digits.pop().unwrap()]
                } else {
                    let u = digits.pop().unwrap();
                    let v = digits.pop().unwrap();

                    vec![v, u]
                }
            }
            .from_digits();

            let inst = self.instructions.get(&opcode).unwrap();

            // collect the arguments
            let args = self.state.get_parameters(inst.parameters);

            // Now for each argument, determine its mode
            let mut ic_args = Vec::with_capacity(args.len());

            for arg in args {
                // Get the corresopnding parameter mode specifier
                let parameter_mode = {
                    if let Some(mode) = digits.pop() {
                        mode
                    } else {
                        0
                    }
                };

                // Add the argument
                ic_args.push(ICCode {
                    value: arg,
                    mode: ICMode::from(parameter_mode),
                });
            }

            let inst = self.instructions.get_mut(&opcode).unwrap();

            // evaluate the instruction
            let result = (inst.evaluate)(&mut self.state, ic_args);

            // Update the instruction pointer
            match result {
                ICFinalization::Continue => {
                    self.state.jump_by(inst.parameters + 1);
                }
                ICFinalization::NoMove => {}
                ICFinalization::Terminate => {
                    self.state.jump_by(inst.parameters + 1);
                    break;
                }
            }
        }

        ICTerminalState {
            state: self.state.clone(),
            last_instruction: opcode as usize,
        }
    }

    pub fn run(&mut self, inputs: Vec<i64>) -> ICTerminalState {
        let inputs = Rc::new(RwLock::new(inputs.into()));
        self.run_with_inputs(&inputs)
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

#[derive(Clone, Debug)]
pub struct InterpreterInput {
    pub buffer: VecDeque<i64>,
}

impl InterpreterInput {
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

    pub fn add(&mut self, input: i64) {
        self.buffer.push_back(input);
    }

    pub fn pop(&mut self) -> i64 {
        self.buffer.pop_front().unwrap()
    }
}

impl Default for InterpreterInput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<i64>> for InterpreterInput {
    fn from(buffer: Vec<i64>) -> Self {
        Self {
            buffer: buffer.into_iter().collect(),
        }
    }
}
