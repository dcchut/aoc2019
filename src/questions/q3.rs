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

#[test]
fn test_intersections() {
    let input1 = ProblemInput::from(vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83",
    ]);

    let input2 = ProblemInput::from(vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    ]);

    let q3 = Q3 {};
    assert_eq!(q3.part1(&input1), 159);
    assert_eq!(q3.part1(&input2), 135);

    assert_eq!(q3.part2(&input1), 610);
    assert_eq!(q3.part2(&input2), 410);
}
