use aoc2019::{FromLines, Solution};

// TODO: figure out a reasonable directory structure, don't want every solution
// lurking in this file
pub struct Q1;

impl Solution for Q1 {
    fn part1(&self, lines: &[&str]) -> i64 {
        // TODO: `FromLines` api could be changed to something like:
        // ```
        // let mut buffer : Vec<i64> = Vec::new();
        // lines.extract(&mut buffer);
        // ```
        Vec::from_lines(lines).into_iter().map(required_fuel).sum()
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        Vec::from_lines(lines)
            .into_iter()
            .map(required_fuel_recursive)
            .sum()
    }
}

fn required_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn required_fuel_recursive(mass: i64) -> i64 {
    // The solution is found so quick it seems a bit unnecessary to do any caching.
    let mut total = 0;
    let mut current_mass = required_fuel(mass);

    while current_mass > 0 {
        total += current_mass;
        current_mass = required_fuel(current_mass);
    }

    total
}

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
