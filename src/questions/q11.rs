use aoc2019::grid::{Direction, Position};
use aoc2019::ic::interpreter::ICInterpreter;
use aoc2019::ic::io::Queue;
use aoc2019::ic::ICPostAction;
use aoc2019::{Extract, ProblemInput, Solution};
use std::cmp::{max, min};
use std::collections::HashMap;

fn paint_ship(lines: &ProblemInput, starting_color: i64) -> HashMap<Position, i64> {
    let mut interpreter: ICInterpreter = lines.extract().unwrap();
    // on input and output we want to halt program execution
    interpreter.postprocess(3, |_, fz| {
        if let ICPostAction::Continue = fz {
            *fz = ICPostAction::Terminate;
        }
    });
    interpreter.postprocess(4, |_, fz| {
        if let ICPostAction::Continue = fz {
            *fz = ICPostAction::Terminate;
        }
    });

    let mut paint = HashMap::new();
    let mut current_position = Position::new(0, 0);
    let mut current_direction = Direction::Up;
    paint.insert(current_position, starting_color);

    loop {
        let current_input = paint.get(&current_position).cloned().unwrap_or(0);

        // run the interpreter until it halts
        interpreter.run_with_inputs(vec![current_input]);

        let state = interpreter.terminal_state();

        // reached an input instruction, so prime the interpreter with another input
        if state.opcode == 3 {
            continue;
        } else if state.opcode == 99 {
            break; // we're done here
        }

        // must be an output instruction
        assert_eq!(state.opcode, 4);

        // run the interpreter until we get the second output
        interpreter.run_with_inputs(vec![current_input]);

        // another output instruction
        assert_eq!(interpreter.terminal_state().opcode, 4);

        let color = interpreter.outputs.pop().unwrap();
        let direction = interpreter.outputs.pop().unwrap();

        paint.insert(current_position, color);

        if direction == 0 {
            current_direction = current_direction.left();
        } else if direction == 1 {
            current_direction = current_direction.right();
        } else {
            panic!("invalid direction {} found", direction);
        }

        // now move
        current_position = current_position.go(current_direction);
    }

    paint
}

pub struct Q11;

impl Solution for Q11 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        paint_ship(lines, 0).len() as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let painting = paint_ship(lines, 1);

        // find some bounds on the position
        let mut minx = 999_999;
        let mut maxx = -999_999;
        let mut miny = 999_999;
        let mut maxy = -999_999;

        for pos in painting.keys() {
            minx = min(minx, pos.x);
            maxx = max(maxx, pos.x);
            miny = min(miny, pos.y);
            maxy = max(maxy, pos.y);
        }

        // The coordinate system is upside down, so reverse our y direction.
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                if painting.get(&Position::new(x, y)).cloned().unwrap_or(1) == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        0
    }
}
