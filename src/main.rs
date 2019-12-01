use crate::questions::*;
use aoc2019::Solution;

mod questions;

fn main() {
    // TODO: make a macro to do this for me
    let solutions: Vec<Box<dyn Solution>> = vec![Box::new(Q1 {})];

    let reply = rprompt::prompt_reply_stdout("Problem: ").unwrap();

    if let Ok(index) = reply.parse::<usize>() {
        let index = index - 1;

        if index < solutions.len() {
            // read the input data
            let path = format!("data/q{}.txt", index + 1);

            // TODO: replace Vec<String> with ProblemInput struct
            let lines: Vec<String> = std::fs::read_to_string(path)
                .expect("failed to read problem input file")
                .lines()
                .map(String::from)
                .collect();

            let line_ref: Vec<&str> = lines.iter().map(|s| s.as_str()).collect::<Vec<_>>();

            println!("Part 1: {}", solutions[index].part1(&line_ref));
            println!("Part 2: {}", solutions[index].part2(&line_ref));
        }
    }
}
