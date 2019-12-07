use aoc2019::ic::InterpreterInput;
use aoc2019::{Extract, ICInterpreter, ProblemInput, Solution};
use std::cmp::max;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::RwLock;

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
                            let stage_1 = interpreter.run(vec![i, 0]);
                            interpreter.reset();
                            let stage_2 = interpreter.run(vec![j, stage_1.outputs[0]]);
                            interpreter.reset();
                            let stage_3 = interpreter.run(vec![k, stage_2.outputs[0]]);
                            interpreter.reset();
                            let stage_4 = interpreter.run(vec![l, stage_3.outputs[0]]);
                            interpreter.reset();
                            let stage_5 = interpreter.run(vec![m, stage_4.outputs[0]]);

                            if stage_5.outputs[0] > best_output {
                                best_output = stage_5.outputs[0];
                            }
                        }
                    }
                }
            }
        }

        best_output
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter1: ICInterpreter = lines.extract().unwrap();
        let mut interpreter2: ICInterpreter = lines.extract().unwrap();
        let mut interpreter3: ICInterpreter = lines.extract().unwrap();
        let mut interpreter4: ICInterpreter = lines.extract().unwrap();
        let mut interpreter5: ICInterpreter = lines.extract().unwrap();

        let mut best_output = -1;

        for i in 5..=9 {
            for j in 5..=9 {
                if i == j {
                    continue;
                }
                for k in 5..=9 {
                    if i == k || j == k {
                        continue;
                    }
                    for l in 5..=9 {
                        if i == l || j == l || k == l {
                            continue;
                        }
                        for m in 5..=9 {
                            if i == m || j == m || k == m || l == m {
                                continue;
                            }
                            interpreter1.reset();
                            interpreter2.reset();
                            interpreter3.reset();
                            interpreter4.reset();
                            interpreter5.reset();

                            let input1 = Rc::new(RwLock::new(InterpreterInput::from(vec![i, 0])));
                            let input2 = Rc::new(RwLock::new(InterpreterInput::single(j)));
                            let input3 = Rc::new(RwLock::new(InterpreterInput::single(k)));
                            let input4 = Rc::new(RwLock::new(InterpreterInput::single(l)));
                            let input5 = Rc::new(RwLock::new(InterpreterInput::single(m)));

                            let last_value;

                            loop {
                                let state = interpreter1.run_with_inputs(&input1);
                                if state.last_instruction == 99 {
                                    last_value =
                                        interpreter1.state.inputs.read().unwrap().buffer[0];
                                    break;
                                }

                                input2
                                    .write()
                                    .unwrap()
                                    .add(interpreter1.state.outputs.remove(0));

                                interpreter2.run_with_inputs(&input2);
                                input3
                                    .write()
                                    .unwrap()
                                    .add(interpreter2.state.outputs.remove(0));

                                interpreter3.run_with_inputs(&input3);
                                input4
                                    .write()
                                    .unwrap()
                                    .add(interpreter3.state.outputs.remove(0));

                                interpreter4.run_with_inputs(&input4);
                                input5
                                    .write()
                                    .unwrap()
                                    .add(interpreter4.state.outputs.remove(0));

                                interpreter5.run_with_inputs(&input5);
                                input1
                                    .write()
                                    .unwrap()
                                    .add(interpreter5.state.outputs.remove(0));
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
