use aoc2019::ic::interpreter::ICInterpreter;
use aoc2019::ic::io::Queue;
use aoc2019::{Extract, ProblemInput, Solution};

pub struct Q9;

impl Solution for Q9 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.run_with_inputs(vec![1]);

        interpreter.outputs.pop().unwrap()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.run_with_inputs(vec![2]);

        interpreter.outputs.pop().unwrap()
    }
}
