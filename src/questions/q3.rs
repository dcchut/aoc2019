use aoc2019::{Extract, ProblemInput, Solution};

pub struct Q3;

use aoc2019::grid::{Grid, HistoryVisitor, Position, StepVisitor};
use std::collections::{HashMap, HashSet};

impl Solution for Q3 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let (path1, path2) = lines.extract().unwrap();

        let grid1 = Grid::new(HistoryVisitor::new());
        let grid2 = Grid::new(HistoryVisitor::new());
        let history1 = grid1.go_many(path1);
        let history2 = grid2.go_many(path2);

        // Find the smallest L1 size for a non-origin intersection point
        history1
            .intersection(&history2)
            .map(|pos| pos.l1())
            .filter(|&size| size > 0)
            .min()
            .unwrap()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let (path1, path2) = lines.extract().unwrap();

        let grid1 = Grid::new(StepVisitor::new());
        let grid2 = Grid::new(StepVisitor::new());
        let step_history1: HashMap<Position, usize> = grid1.go_many(path1);
        let step_history2: HashMap<Position, usize> = grid2.go_many(path2);

        step_history1
            .keys()
            .collect::<HashSet<_>>()
            .intersection(&step_history2.keys().collect::<HashSet<_>>())
            .map(|key| step_history1[*key] + step_history2[*key])
            .filter(|&steps| steps > 0)
            .min()
            .unwrap() as i64
    }
}
