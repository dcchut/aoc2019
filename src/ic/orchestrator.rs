use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::{ICInput, Queue};
use crate::ic::state::ICTerminalState;

pub struct ICInterpreterOrchestrator {
    pub interpreters: Vec<ICInterpreter>,
    pub current_interpreter: usize,
}

impl ICInterpreterOrchestrator {
    pub fn new(interpreters: Vec<ICInterpreter>) -> Self {
        Self {
            interpreters,
            current_interpreter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.interpreters
            .iter_mut()
            .for_each(|interpreter| interpreter.reset());
        self.current_interpreter = 0;
    }

    pub fn prime(&mut self, inputs: Vec<ICInput>) {
        for (index, input) in inputs.into_iter().enumerate() {
            self.interpreters[index].inputs = input;
        }
    }

    pub fn run(&mut self) -> ICTerminalState<'_> {
        let current_index = self.current_interpreter;
        let next_index = (self.current_interpreter + 1) % self.interpreters.len();

        // Run the current interpreter, retrieving its first output
        let output = {
            let current_interpreter = &mut self.interpreters[current_index];
            current_interpreter.run();
            current_interpreter.outputs.pop()
        };

        // Add the received output to the input of the next interpreter
        if let Some(input) = output {
            self.interpreters[next_index].inputs.add(input);
        }

        // Update the current interpreter pointer
        self.current_interpreter = next_index;

        // Return the terminal state object of the (now previous) interpreter
        self.interpreters[current_index].terminal_state()
    }
}
