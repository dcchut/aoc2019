use aoc2019::ic::{ICFinalization, ICInput, ICInterpreterOrchestrator};
use aoc2019::{Extract, ICInterpreter, ProblemInput, Solution};
use std::cmp::max;
use std::collections::HashSet;

pub struct Q7;

impl Solution for Q7 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        let mut best_output = -1;

        for i in 0..=4 {
            for j in 0..=4 {
                for k in 0..=4 {
                    for l in 0..=4 {
                        for m in 0..=4 {
                            let index_set: HashSet<_> = vec![i, j, k, l, m].into_iter().collect();
                            if index_set.len() != 5 {
                                continue;
                            }

                            // Run the interpreter
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![i, 0]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![j, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![k, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![l, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![m, last]);
                            let last = interpreter.outputs.pop().unwrap();

                            best_output = max(best_output, last);
                        }
                    }
                }
            }
        }

        best_output
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        // Orchestrate the interpreters
        let mut orchestrators = ICInterpreterOrchestrator::new(vec![lines.extract().unwrap(); 5]);

        // register this postprocess with each interpreter
        for index in 0..5 {
            orchestrators.interpreters[index].postprocess(4, |_, fz: &mut ICFinalization| {
                // convert output finalization continue states to terminate
                match fz {
                    ICFinalization::Continue => {
                        *fz = ICFinalization::Terminate;
                    }
                    _ => {}
                };
            });
        }

        let mut best_output = -1;

        for i in 5..=9 {
            for j in 5..=9 {
                for k in 5..=9 {
                    for l in 5..=9 {
                        for m in 5..=9 {
                            let index_set: HashSet<_> = vec![i, j, k, l, m].into_iter().collect();
                            if index_set.len() != 5 {
                                continue;
                            }

                            orchestrators.reset();
                            orchestrators.prime(vec![
                                ICInput::from(vec![i, 0]),
                                ICInput::single(j),
                                ICInput::single(k),
                                ICInput::single(l),
                                ICInput::single(m),
                            ]);

                            let last_value;

                            loop {
                                let state = orchestrators.run();
                                if state.opcode == 99 {
                                    last_value = orchestrators.interpreters[0].inputs.buffer[0];
                                    break;
                                }
                                orchestrators.run();
                                orchestrators.run();
                                orchestrators.run();
                                orchestrators.run();
                            }

                            best_output = max(best_output, last_value);
                        }
                    }
                }
            }
        }

        best_output
    }
}
