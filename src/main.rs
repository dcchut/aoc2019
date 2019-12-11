use crate::questions::*;
use anyhow::Result;
use aoc2019::{ProblemInput, Solution};
use crossbeam::thread;

mod questions;

fn run_problem(solutions: &'static [Box<dyn Solution>], index: usize) -> Result<(i64, i64)> {
    let (part1, part2) = thread::scope(move |s| {
        let solution = &solutions[index];

        let part1 = s
            .spawn(move |_| {
                let path = format!("data/q{}.txt", index + 1);
                let problem_input = ProblemInput::new(path).unwrap();

                solution.part1(&problem_input)
            })
            .join();

        let part2 = s
            .spawn(move |_| {
                let path = format!("data/q{}.txt", index + 1);
                let problem_input = ProblemInput::new(path).unwrap();

                solution.part2(&problem_input)
            })
            .join();

        (part1, part2)
    })
    .unwrap();

    Ok((part1.unwrap(), part2.unwrap()))
}

fn check_outputs(solutions: &'static [Box<dyn Solution>]) {
    assert_eq!(run_problem(solutions, 0).unwrap(), (3363760, 5042767));
    assert_eq!(run_problem(solutions, 1).unwrap(), (3058646, 8976));
    assert_eq!(run_problem(solutions, 2).unwrap(), (5357, 101956));
    assert_eq!(run_problem(solutions, 3).unwrap(), (966, 628));
    assert_eq!(run_problem(solutions, 4).unwrap(), (2845163, 9436229));
    assert_eq!(run_problem(solutions, 5).unwrap(), (387356, 532));
    assert_eq!(run_problem(solutions, 6).unwrap(), (17406, 1047153));
    assert_eq!(run_problem(solutions, 7).unwrap(), (1064, 0));
    assert_eq!(run_problem(solutions, 8).unwrap(), (2745604242, 51135));
}

fn main() -> Result<()> {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Q1 {}),
        Box::new(Q2 {}),
        Box::new(Q3 {}),
        Box::new(Q4 {}),
        Box::new(Q5 {}),
        Box::new(Q6 {}),
        Box::new(Q7 {}),
        Box::new(Q8 {}),
        Box::new(Q9 {}),
        Box::new(Q10 {}),
        Box::new(Q11 {}),
    ];

    let solutions: &'static [Box<dyn Solution>] = Box::leak(solutions.into_boxed_slice());

    let reply = rprompt::prompt_reply_stdout("Problem: ")?;

    if let Ok(index) = reply.parse::<usize>() {
        if index == 0 {
            check_outputs(solutions);
        } else {
            let index = index - 1;

            if index < solutions.len() {
                let (part1, part2) = run_problem(solutions, index)?;

                println!("Part 1: {}", part1);
                println!("Part 2: {}", part2);
            }
        }
    }

    Ok(())
}
