#[allow(unused_imports)]
use aoc2019::{Digits, Extract, FromDigits, ProblemInput, Solution};

pub struct Q8;

impl Solution for Q8 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let digits = lines.digits();

        let width = 25;
        let height = 6;

        let mut layer = Vec::new();
        let mut zeros = 0;
        let mut best_zeros = 999_999_999;
        let mut best_layer = vec![];

        for digit in digits {
            if digit == 0 {
                zeros += 1;
            }
            layer.push(digit);

            if layer.len() == width * height {
                if zeros < best_zeros {
                    best_zeros = zeros;
                    best_layer = layer.clone();
                }
                layer.clear();
                zeros = 0;
            }
        }

        let mut ones = 0;
        let mut twos = 0;

        for i in best_layer {
            if i == 1 {
                ones += 1;
            } else if i == 2 {
                twos += 1;
            }
        }

        ones * twos
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let digits = lines.digits();

        let width = 25;
        let height = 6;

        let mut pixels = vec![3; width * height];
        let mut ix = 0;

        while ix < digits.len() {
            for x in 0..width {
                for y in 0..height {
                    let c = (x * height) + y;

                    if pixels[c] == 3 || pixels[c] == 2 {
                        pixels[c] = digits[ix];
                    }
                    ix += 1;
                }
            }
        }

        // Pretty formatting
        for y in 0..height {
            for x in 0..width {
                if pixels[y * width + x] == 1 {
                    print!("1");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        0
    }
}
