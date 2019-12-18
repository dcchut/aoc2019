use crate::grid::Direction;
use crate::grid::Position;
use crate::ic::interpreter::ICInterpreter;
use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct ScaffoldMap {
    pub height: i64,
    pub width: i64,
    pub robot: (Position, Direction),
    pub scaffolds: HashSet<Position>,
}

impl ScaffoldMap {
    fn adjacent_positions(x: i64, y: i64) -> Vec<Position> {
        vec![
            Position::new(x - 1, y),
            Position::new(x + 1, y),
            Position::new(x, y - 1),
            Position::new(x, y + 1),
        ]
    }

    pub fn intersections(&self) -> Vec<Position> {
        let mut intersections = Vec::new();

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let mut intersection_point = true;

                for position in Self::adjacent_positions(x, y) {
                    if !self.scaffolds.contains(&position) {
                        intersection_point = false;
                        break;
                    }
                }

                if intersection_point {
                    intersections.push(Position::new(x, y));
                }
            }
        }

        intersections
    }
}

impl Extract<ScaffoldMap> for (&ProblemInput, i64) {
    fn extract(&self) -> Result<ScaffoldMap> {
        let mut interpreter: ICInterpreter = self.0.extract().unwrap();
        interpreter.state.memory[0] = self.1;
        interpreter.run();

        let characters = interpreter
            .outputs
            .outputs
            .iter()
            .map(|v| char::from(*v as u8))
            .collect::<String>()
            .split('\n')
            .filter_map(|v| {
                if v.len() > 0 {
                    Some(v.chars().collect::<Vec<_>>())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let width = characters[0].len() as i64;
        let height = characters.len() as i64;
        let mut scaffolds = HashSet::new();
        let mut robot = None;

        for y in 0..height {
            for x in 0..width {
                let c = characters[y as usize][x as usize];

                if c == '.' {
                    continue;
                }

                scaffolds.insert(Position::new(x, y));

                if c == '#' {
                    continue;
                }

                robot = Some((
                    Position::new(x, y),
                    match c {
                        '^' => Direction::Up,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        token => panic!("invalid token {} encountered", token),
                    },
                ));
            }
        }

        Ok(ScaffoldMap {
            height,
            width,
            robot: robot.unwrap(),
            scaffolds,
        })
    }
}

pub struct Q17;

fn ord(x: char) -> u8 {
    x as u8
}

impl Solution for Q17 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let scaffold: ScaffoldMap = (lines, 1).extract().unwrap();

        scaffold
            .intersections()
            .into_iter()
            .map(|pos| pos.x * pos.y)
            .sum()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();

        interpreter.state.memory[0] = 2;
        let inputs: Vec<i64> = vec![
            ord('A'),
            ord(','),
            ord('B'),
            ord(','),
            ord('A'),
            ord(','),
            ord('C'),
            ord(','),
            ord('A'),
            ord(','),
            ord('B'),
            ord(','),
            ord('C'),
            ord(','),
            ord('B'),
            ord(','),
            ord('C'),
            ord(','),
            ord('B'),
            ord('\n'),
            ord('R'),
            ord(','),
            ord('8'),
            ord(','),
            ord('L'),
            ord(','),
            ord('1'),
            ord('0'),
            ord(','),
            ord('L'),
            ord(','),
            ord('1'),
            ord('2'),
            ord(','),
            ord('R'),
            ord(','),
            ord('4'),
            ord('\n'),
            ord('R'),
            ord(','),
            ord('8'),
            ord(','),
            ord('L'),
            ord(','),
            ord('1'),
            ord('2'),
            ord(','),
            ord('R'),
            ord(','),
            ord('4'),
            ord(','),
            ord('R'),
            ord(','),
            ord('4'),
            ord('\n'),
            ord('R'),
            ord(','),
            ord('8'),
            ord(','),
            ord('L'),
            ord(','),
            ord('1'),
            ord('0'),
            ord(','),
            ord('R'),
            ord(','),
            ord('8'),
            ord('\n'),
            ord('n'),
            ord('\n'),
        ]
        .into_iter()
        .map(|v| v as i64)
        .collect();

        interpreter.run_with_inputs(inputs);

        // TODO: encapuslate better
        interpreter.outputs.outputs.pop_back().unwrap()
    }
}
