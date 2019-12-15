use crate::ic::interpreter::ICInterpreter;
use crate::{Extract, ProblemInput, Solution};

pub struct Q5;

impl Solution for Q5 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        run(lines, 1)
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        run(lines, 5)
    }
}

fn run(lines: &ProblemInput, input: i64) -> i64 {
    let mut interpreter: ICInterpreter = lines.extract().unwrap();
    interpreter.run_with_inputs(vec![input]);
    interpreter.outputs.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ic::io::Queue;
    use crate::load_problem_input;

    #[test]
    fn test_repeat_input() {
        // Should always just output whatever was input
        let input = ProblemInput::from(vec!["3,0,4,0,99"]);
        let mut interpreter: ICInterpreter = input.extract().unwrap();
        interpreter.run_with_inputs(vec![7]);

        assert_eq!(interpreter.outputs.pop(), Some(7));

        interpreter.reset();

        interpreter.run_with_inputs(vec![9]);
        assert_eq!(interpreter.outputs.pop(), Some(9));
    }

    #[test]
    fn test_interpreter_modes() {
        // Test immediate vs. position mode
        let input = ProblemInput::from(vec!["1002,4,3,4,33"]);
        let mut interpreter: ICInterpreter = input.extract().unwrap();
        interpreter.run();

        assert_eq!(interpreter.terminal_state().memory, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_comparison_operators() {
        // Tests how large a number is in comparison to 8
        let input = ProblemInput::from(vec!["3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"]);
        let mut interpreter: ICInterpreter = input.extract().unwrap();
        interpreter.run_with_inputs(vec![7]);

        assert_eq!(interpreter.outputs.pop(), Some(999));
        interpreter.reset();

        interpreter.run_with_inputs(vec![8]);
        assert_eq!(interpreter.outputs.pop(), Some(1000));
        interpreter.reset();

        interpreter.run_with_inputs(vec![9]);
        assert_eq!(interpreter.outputs.pop(), Some(1001));
    }

    #[test]
    fn test_part1_solution() {
        let q5 = Q5;
        assert_eq!(q5.part1(&load_problem_input(5)), 2_845_163);
    }

    #[test]
    fn test_part2_solution() {
        let q5 = Q5;
        assert_eq!(q5.part2(&load_problem_input(5)), 9_436_229);
    }
}
