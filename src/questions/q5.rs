use aoc2019::{Extract, ICInterpreter, ProblemInput, Solution};

pub struct Q5;

impl Solution for Q5 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();

        *interpreter.run(vec![1]).outputs.last().unwrap()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();

        *interpreter.run(vec![5]).outputs.last().unwrap()
    }
}

#[test]
fn test_funky_interpreter() {
    // Should always just output whatever was input
    let input = ProblemInput::from(vec!["3,0,4,0,99"]);
    let mut interpreter: ICInterpreter = input.extract().unwrap();
    assert_eq!(interpreter.run(vec![7]).outputs, vec![7]);
    interpreter.reset();
    assert_eq!(interpreter.run(vec![9]).outputs, vec![9]);

    // Test immediate vs. position mode
    let input = ProblemInput::from(vec!["1002,4,3,4,33"]);
    let mut interpreter: ICInterpreter = input.extract().unwrap();
    assert_eq!(interpreter.run(vec![]).memory, vec![1002, 4, 3, 4, 99]);

    // Tests how large a number is in comparison to 8
    let input = ProblemInput::from(vec!["3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"]);
    let mut interpreter: ICInterpreter = input.extract().unwrap();
    assert_eq!(interpreter.run(vec![7]).outputs, vec![999]);
    interpreter.reset();
    assert_eq!(interpreter.run(vec![8]).outputs, vec![1000]);
    interpreter.reset();
    assert_eq!(interpreter.run(vec![9]).outputs, vec![1001]);
}
