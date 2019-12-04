use crate::questions::*;
use anyhow::Result;
use aoc2019::{ProblemInput, Solution};

mod questions;

fn main() -> Result<()> {
    // TODO: make a macro to do  this for me
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Q1 {}),
        Box::new(Q2 {}),
        Box::new(Q3 {}),
        Box::new(Q4 {}),
    ];

    let reply = rprompt::prompt_reply_stdout("Problem: ")?;

    if let Ok(index) = reply.parse::<usize>() {
        let index = index - 1;

        if index < solutions.len() {
            // read the input data
            let path = format!("data/q{}.txt", index + 1);

            let problem_input = ProblemInput::new(path)?;

            println!("Part 1: {}", solutions[index].part1(&problem_input));
            println!("Part 2: {}", solutions[index].part2(&problem_input));
        }
    }

    Ok(())
}
