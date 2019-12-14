use crate::{ProblemInput, Solution};

pub struct Q1;

impl Solution for Q1 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        lines.as_vec().into_iter().map(required_fuel).sum()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        lines
            .as_vec()
            .into_iter()
            .map(required_fuel_recursive)
            .sum()
    }
}

fn required_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn required_fuel_recursive(mass: i64) -> i64 {
    let mut total = 0;
    let mut mass = required_fuel(mass);

    while mass > 0 {
        total += mass;
        mass = required_fuel(mass);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_required_fuel() {
        // A few basic test cases for part 1 solution
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }

    #[test]
    fn test_required_fuel_recursive() {
        // A few basic test cases for part 2 solution
        assert_eq!(required_fuel_recursive(14), 2);
        assert_eq!(required_fuel_recursive(1969), 966);
        assert_eq!(required_fuel_recursive(100756), 50346);
    }

    #[test]
    fn test_part1_solution() {
        let q1 = Q1;
        assert_eq!(q1.part1(&load_problem_input(1)), 3_363_760);
    }

    #[test]
    fn test_part2_solution() {
        let q1 = Q1;
        assert_eq!(q1.part2(&load_problem_input(1)), 5_042_767);
    }
}
