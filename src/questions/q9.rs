use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::Queue;
use crate::{Extract, ProblemInput, Solution};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q9 = Q9;
        assert_eq!(q9.part1(&load_problem_input(9)), 2_745_604_242);
    }

    #[test]
    fn test_part2_solution() {
        let q9 = Q9;
        assert_eq!(q9.part2(&load_problem_input(9)), 51_135);
    }
}
