use crate::{FromDigits, ProblemInput, Solution};

pub struct Q16;

const D_VAL: [i64; 4] = [1, 1, -1, -1];

#[inline(always)]
fn fast_fft(input: &mut [i64], offset: usize) {
    let mut sum = 0;
    let mut tmp = vec![0; input.len()];

    for i in (offset..input.len()).rev() {
        sum += input[i];
        tmp[i - offset] = sum;

        let mut acc_sum = sum;

        let mut j = 2;
        while input.len() > j * (i + 1) - 1 {
            acc_sum += tmp[j * (i + 1) - 1 - offset] * D_VAL[j & 0b11];
            j += 1;
        }

        input[i] = (acc_sum.abs() % 10) as i64;
    }
}

impl Solution for Q16 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut digits = lines
            .digits()
            .into_iter()
            .map(|v| v as i64)
            .collect::<Vec<_>>();

        for _ in 0..100 {
            fast_fft(&mut digits, 0);
        }

        (&digits[0..8]).from_digits()
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let digits = lines
            .digits()
            .into_iter()
            .map(|v| v as i64)
            .collect::<Vec<_>>();

        let offset = (&digits[0..7]).from_digits() as usize;

        let mut expanded_input = Vec::with_capacity(digits.len() * 10000);

        for _ in 0..10000 {
            expanded_input.extend_from_slice(&digits);
        }

        for _ in 0..100 {
            fast_fft(&mut expanded_input, offset);
        }

        (&expanded_input[offset..(offset + 8)]).from_digits()
    }
}
