use crate::{ProblemInput, Solution};

pub struct Q4;

fn satisfying_numbers(min: usize, max: usize, isolate: bool) -> Vec<usize> {
    let mut numbers = Vec::new();

    for number in min..=max {
        let digits = number
            .to_string()
            .chars()
            .map(|v| v.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        for i in 0..5 {
            // repeated digit
            if digits[i] == digits[i + 1] {
                // increasing digits
                if digits[0] <= digits[1]
                    && digits[1] <= digits[2]
                    && digits[2] <= digits[3]
                    && digits[3] <= digits[4]
                    && digits[4] <= digits[5]
                {
                    if isolate {
                        let mut curr = false;

                        for j in 0..=3 {
                            if digits[j] == digits[j + 1] && digits[j + 1] != digits[j + 2] {
                                // we've done it
                                if j > 0 {
                                    if digits[j - 1] != digits[j] {
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
        satisfying_numbers(264_793, 803_935, false).len() as i64
    }

    fn part2(&self, _lines: &ProblemInput) -> i64 {
        satisfying_numbers(264_793, 803_935, true).len() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;
    #[test]
    fn test_satisfying_numbers() {
        let s1 = satisfying_numbers(100_000, 999_999, false);
        let s2 = satisfying_numbers(100_000, 999_999, true);

        assert!(s1.contains(&111_111));
        assert!(!s1.contains(&223_450));
        assert!(!s1.contains(&123_789));

        assert!(s2.contains(&112_223));
        assert!(!s2.contains(&123_444));
        assert!(s2.contains(&111_122));
    }
    #[test]
    fn test_part1_solution() {
        let q4 = Q4;
        assert_eq!(q4.part1(&load_problem_input(4)), 966);
    }

    #[test]
    fn test_part2_solution() {
        let q4 = Q4;
        assert_eq!(q4.part2(&load_problem_input(3)), 628);
    }
}
