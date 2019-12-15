use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::Queue;
use crate::{Extract, ProblemInput, Solution};
use std::collections::HashMap;

pub struct Q13;

impl Solution for Q13 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.run();
        interpreter
            .outputs
            .outputs
            .into_iter()
            .skip(2)
            .step_by(3)
            .filter(|&code| code == 2)
            .count() as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.state.memory[0] = 2;

        // halt the interpreter before every input instruction
        interpreter.yields.insert(3);

        let mut last_ball = (19, 14);
        let mut ball = (-1, -1);
        let mut paddle = (-1, -1);
        let mut map = HashMap::new();

        loop {
            // display the initial state
            interpreter.run();

            let mut score = -1;

            while !interpreter.outputs.outputs.is_empty() {
                let x = interpreter.outputs.pop().unwrap();
                let y = interpreter.outputs.pop().unwrap();

                if x == -1 && y == 0 {
                    score = interpreter.outputs.pop().unwrap();
                } else {
                    let code = match interpreter.outputs.pop().unwrap() {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => {
                            paddle = (x, y);
                            Tile::Paddle
                        }
                        4 => {
                            ball = (x, y);
                            Tile::Ball
                        }
                        _ => panic!(),
                    };

                    map.insert((x, y), code);
                }
            }

            // If there are no blocks left in our map, then we're done.
            if !map.values().any(|&x| x == Tile::Block) {
                return score;
            }

            // make a guess at where the ball is headed
            let direction = if ball.0 > last_ball.0 { 1 } else { -1 };
            let vdirection = if ball.1 > last_ball.1 { 1 } else { -1 };

            let target = if direction == 1 && vdirection == 1 {
                // if the ball is moving right + down, then we can easily determine where we need to be
                ball.0 + (17 - ball.1)
            } else if direction == -1 && vdirection == 1 {
                // similarly if the ball is moving left + down
                ball.0 - (17 - ball.1)
            } else if direction == 1 && vdirection == -1 {
                // if the ball is moving right and up, then it's a little tricky to figure out where we need to go, so just go right.
                paddle.0 + 1
            } else {
                // similarly it's difficult to figure out where we need to go, so just move left.
                paddle.0 - 1
            };

            last_ball = ball;

            interpreter.inputs.add(if paddle.0 < target {
                1
            } else if paddle.0 > target {
                -1
            } else {
                0
            });
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Wall,   // Indestructible
    Block,  // Can be broken
    Paddle, // Indestructible
    Ball,   // Moves diagonally and bounces off objects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q13 = Q13;
        assert_eq!(q13.part1(&load_problem_input(13)), 200);
    }

    #[test]
    fn test_part2_solution() {
        let q13 = Q13;
        assert_eq!(q13.part2(&load_problem_input(13)), 9_803);
    }
}
