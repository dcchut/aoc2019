use aoc2019::{ProblemInput, Solution};

pub struct Q4;

fn satisfying_numbers(min: usize, max: usize, isolate: bool) -> Vec<usize> {
    let mut numbers = Vec::new();

    for number in min..=max {
        let digits = number.to_string().chars().map(|v| v.to_digit(10).unwrap()).collect::<Vec<_>>();

        for i in 0..5 {
            // repeated digit
            if digits[i] == digits[i+1] {
                // increasing digits
                if digits[0] <= digits[1] && digits[1] <= digits[2] && digits[2] <= digits[3] && digits[3] <= digits[4] && digits[4] <= digits[5] {
                    if isolate {
                        let mut curr = false;

                        for j in 0..=3 {
                            if digits[j] == digits[j+1] && digits[j+1] != digits[j+2] {
                                // we've done it
                                if j > 0 {
                                    if digits[j-1] != digits[j] {
                                        curr = true;
                                    }
                                } else {
                                    curr = true;
                                }
                            }
                        }
    
                        // check last position
                        if digits[3] != digits[4] && digits[4] == digits[5] {
                            curr = true;
                        }
    
                        if curr {
                            numbers.push(number);
                        }
                    } else {
                        numbers.push(number);
                    }
                }
                break;
            }
        }
    }

    numbers
}

impl Solution for Q4 {
    fn part1(&self, _lines: &ProblemInput) -> i64 {
        satisfying_numbers(264793, 803935, false).len() as i64
    }

    fn part2(&self, _lines: &ProblemInput) -> i64 {
        satisfying_numbers(264793, 803935, true).len() as i64
    }
}
