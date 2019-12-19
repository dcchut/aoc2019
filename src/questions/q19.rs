use crate::grid::Position;
use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::Queue;
use crate::ic::ICPostAction;
use crate::{Extract, ProblemInput, Solution};

pub struct Q19;

fn run_at(x: i64, y: i64, interpreter: &mut ICInterpreter) -> bool {
    interpreter.reset();
    interpreter.run_with_inputs(vec![x, y]);
    interpreter.outputs.pop().unwrap() == 1
}

fn contains_square_at_position(x: i64, y: i64, interpreter: &mut ICInterpreter) -> bool {
    run_at(x, y, interpreter)
        && run_at(x + 99, y, interpreter)
        && run_at(x, y + 99, interpreter)
        && run_at(x + 99, y + 99, interpreter)
}

impl Solution for Q19 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.postprocess(4, |_, fz: &mut ICPostAction| {
            // convert output finalization continue states to terminate
            if let ICPostAction::Continue = fz {
                *fz = ICPostAction::Terminate;
            };
        });

        let mut count = 0;

        for x in 0..50 {
            for y in 0..50 {
                interpreter.run_with_inputs(vec![x, y]);
                count += interpreter.outputs.pop().unwrap();
                interpreter.reset();
            }
        }

        count
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        // starting at (50, 50) ->
        // find the first x value
        //        let xmin = rprompt::prompt_reply_stdout("XMIN: ")?;
        //        let xmax = rprompt::prompt_reply_stdout("XMAX: ")?;
        //        let ymin = rprompt::prompt_reply_stdout("YMIN: ")?;
        //        let ymax = rprompt::pr
        //
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        interpreter.postprocess(4, |_, fz: &mut ICPostAction| {
            // convert output finalization continue states to terminate
            if let ICPostAction::Continue = fz {
                *fz = ICPostAction::Terminate;
            };
        });

        let mut winner = None;

        'outer: for y in 600..800 {
            // Using the power of science, one can see that there will pretty much always be a # at 1.94 * y
            let mut test_x = ((y as f64) * 1.94).ceil() as i64;

            let mut delta = 0;
            // find the left end for this y
            while run_at(test_x - delta, y, &mut interpreter) {
                delta += 1;
            }

            // now find the right end
            let mut right_delta = 0;
            while run_at(test_x + right_delta, y, &mut interpreter) {
                right_delta += 1;
            }

            // now find the first x where we get a square
            for x in (test_x - delta)..=(test_x + right_delta) {
                if contains_square_at_position(x, y, &mut interpreter) {
                    winner = Some((x, y));
                    break 'outer;
                }
            }
        }

        let (x, y) = winner.unwrap();

        x * 10000 + y
    }
}
