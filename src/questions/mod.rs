use aoc2019::{FromLines, Solution};

pub struct Q2;

impl Q2 {
    pub fn run_computer(mut line: Vec<i64>, noun: i64, verb: i64) -> i64 {
        line[1] = noun;
        line[2] = verb;

        let mut position = 0;

        while line[position] != 99 {
            let operation = line[position];
            let s = line[line[position + 1] as usize];
            let t = line[line[position + 2] as usize];
            let u = line[position + 3];

            if operation == 1 {
                line[u as usize] = s + t;
            } else if operation == 2 {
                line[u as usize] = s * t;
            } else {
                panic!("invalid opcode");
            }

            position += 4;
        }

        line[0]
    }
}

impl Solution for Q2 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let line: Vec<i64> = lines[0].split(",").map(|v| v.parse().unwrap()).collect();

        Q2::run_computer(line, 12, 2)
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let baseline: Vec<i64> = lines[0].split(",").map(|v| v.parse().unwrap()).collect();

        for noun in 0..100 {
            for verb in 0..100 {
                let line = baseline.clone();

                if Q2::run_computer(line, noun, verb) == 19690720 {
                    return verb + (noun * 100);
                }
            }
        }

        panic!("failed to find solution");
    }
}

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
