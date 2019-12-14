use crate::ic::io::{ICInput, ICOutput, Queue};
use crate::ic::state::{ICState, ICTerminalState};
use crate::ic::{ICCode, ICMode, ICPostAction};
use crate::{Digits, Extract, FromDigits, ProblemInput};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ICYieldState {
    Yielded,
    Unyielded,
}

pub struct ICInterpreter {
    /// The initial state of the interpreter
    initial_state: ICState,

    /// The current state of our interpreter
    pub state: ICState,

    /// The current inputs to our interpreter
    pub inputs: ICInput,

    /// The current outputs of our interpreter
    pub outputs: ICOutput,

    /// A map indicating which instruction corresponds to a given number
    instructions: HashMap<i64, ICInstruction>,

    /// A map indicating what post-processing should be done given a particular opcode
    processing: HashMap<i64, ICPostProcess>,

    /// The last opcode ran
    pub opcode: usize,

    /// A vec containing opcodes at which point the interpreter should yield control back to the caller
    pub yields: HashSet<i64>,

    yield_state: ICYieldState,
}

impl ICInterpreter {
    pub fn postprocess<F>(&mut self, key: i64, f: F)
    where
        F: 'static + Fn(&mut ICState, &mut ICPostAction),
    {
        self.processing.insert(
            key,
            ICPostProcess {
                evaluate: Box::new(f),
            },
        );
    }

    pub fn register<F>(&mut self, key: i64, parameters: usize, f: F)
    where
        F: 'static + Fn(&mut ICState, &mut ICInput, &mut ICOutput, Vec<ICCode>) -> ICPostAction,
    {
        // Box our closure up, together with an assertion that it receives the correct number of arguments
        let evaluate = Box::new(
            move |state: &mut ICState,
                  inputs: &mut ICInput,
                  outputs: &mut ICOutput,
                  args: Vec<ICCode>| {
                assert_eq!(args.len(), parameters);

                f(state, inputs, outputs, args)
            },
        );

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
            inputs: ICInput::new(),
            outputs: ICOutput::new(),
            instructions: HashMap::new(),
            processing: HashMap::new(),
            opcode: 0,
            yields: HashSet::new(),
            yield_state: ICYieldState::Unyielded,
        };

        // Add instruction
        interpreter.register(1, 3, |state, _, _, args| {
            let s = args[0].value;
            let t = args[1].value;

            state.write(args[2].index, s + t);

            ICPostAction::Continue
        });

        // Mul instruction
        interpreter.register(2, 3, |state, _, _, args| {
            let s = args[0].value;
            let t = args[1].value;

            state.write(args[2].index, s * t);

            ICPostAction::Continue
        });

        // Terminate instruction
        interpreter.register(99, 0, |_, _, _, _| ICPostAction::Terminate);

        // Input instruction
        interpreter.register(3, 1, |state, inputs, _, args| {
            state.write(args[0].index, inputs.pop().unwrap());

            ICPostAction::Continue
        });

        // Output instruction
        interpreter.register(4, 1, |_, _, outputs, args| {
            outputs.add(args[0].value);

            ICPostAction::Continue
        });

        // jump-if-true instruction
        interpreter.register(5, 2, |state, _, _, args| {
            let u = args[0].value;
            let v = args[1].value;

            if u != 0 {
                state.ip = v as usize;

                ICPostAction::NoMove
            } else {
                ICPostAction::Continue
            }
        });

        // jump_if_false instruction
        interpreter.register(6, 2, |state, _, _, args| {
            let u = args[0].value;
            let v = args[1].value;

            if u == 0 {
                state.ip = v as usize;

                ICPostAction::NoMove
            } else {
                ICPostAction::Continue
            }
        });

        // lt instruction
        interpreter.register(7, 3, |state, _, _, args| {
            let s = args[0].value;
            let t = args[1].value;
            state.write(args[2].index, if s < t { 1 } else { 0 });

            ICPostAction::Continue
        });

        // eq instruction
        interpreter.register(8, 3, |state, _, _, args| {
            let s = args[0].value;
            let t = args[1].value;
            state.write(args[2].index, if s == t { 1 } else { 0 });

            ICPostAction::Continue
        });

        // relative base offset
        interpreter.register(9, 1, |state, _, _, args| {
            let s = args[0].value;

            state.relative_base += s;

            ICPostAction::Continue
        });

        interpreter
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state.clone();
        self.inputs.reset();
        self.outputs.reset();
    }

    pub fn terminal_state(&self) -> ICTerminalState<'_> {
        ICTerminalState::new(self)
    }

    pub fn run(&mut self) {
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

            // Check for yields
            if self.yield_state == ICYieldState::Unyielded && self.yields.contains(&opcode) {
                // yield control back to the caller
                self.yield_state = ICYieldState::Yielded;
                return;
            }
            self.yield_state = ICYieldState::Unyielded;

            self.opcode = opcode as usize;

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
                ic_args.push(ICCode::new(&self.state, arg, ICMode::from(parameter_mode)));
            }

            let inst = self.instructions.get_mut(&opcode).unwrap();

            // evaluate the instruction
            let mut result = (inst.evaluate)(
                &mut self.state,
                &mut self.inputs,
                &mut self.outputs,
                ic_args,
            );

            // Do some postprocessing
            if let Some(postprocess) = self.processing.get(&opcode) {
                (postprocess.evaluate)(&mut self.state, &mut result);
            };

            // Update the instruction pointer
            match result {
                ICPostAction::Continue => {
                    self.state.jump_by(inst.parameters + 1);
                }
                ICPostAction::NoMove => {}
                ICPostAction::Terminate => {
                    self.state.jump_by(inst.parameters + 1);
                    break;
                }
            }
        }
    }

    pub fn run_with_inputs(&mut self, inputs: Vec<i64>) {
        self.inputs = ICInput::from(inputs);
        self.run();
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

pub struct ICInstruction {
    /// How many parameters this instruction accepts
    parameters: usize,

    /// A function for evaluating the given instruction
    evaluate: Box<dyn Fn(&mut ICState, &mut ICInput, &mut ICOutput, Vec<ICCode>) -> ICPostAction>,
}

pub struct ICPostProcess {
    evaluate: Box<dyn Fn(&mut ICState, &mut ICPostAction)>,
}
