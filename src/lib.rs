use anyhow::{Context, Result};
use std::path::Path;

pub mod grid;
pub mod ic;
pub mod questions;

pub fn load_problem_input(number: usize) -> ProblemInput {
    let path = format!("data/q{}.txt", number);
    ProblemInput::new(path).unwrap()
}

pub trait Extract<T> {
    fn extract(&self) -> Result<T>;
}

#[derive(Debug, Clone)]
pub struct ProblemInput {
    pub lines: Vec<String>,
}

pub trait Digits {
    fn digits(&self) -> Vec<i64>; // TODO: maybe consider a different return type here (usize? digit?)
}

impl Digits for i64 {
    fn digits(&self) -> Vec<i64> {
        self.to_string()
            .chars()
            .map(|v| v.to_digit(10).unwrap() as i64)
            .collect()
    }
}

pub trait FromDigits {
    fn from_digits(&self) -> i64;
}

impl FromDigits for Vec<i64> {
    fn from_digits(&self) -> i64 {
        self.iter()
            .cloned()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap()
    }
}

impl ProblemInput {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Read our input file into a vector of strings
        let lines: Vec<String> = std::fs::read_to_string(path)
            .with_context(|| format!("unable to load problem input from {}", path.display()))?
            .lines()
            .map(String::from)
            .collect();

        Ok(Self { lines })
    }

    // Technically we don't need these functions, but they help get around
    // our type inference issues
    pub fn as_vec(&self) -> Vec<i64> {
        self.extract().unwrap()
    }

    pub fn as_deep_vec(&self) -> Vec<Vec<i64>> {
        self.extract().unwrap()
    }

    pub fn as_csv(&self) -> Vec<String> {
        self.lines
            .iter()
            .map(|line| line.split(',').map(String::from).collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    pub fn digits(&self) -> Vec<u32> {
        self.lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

impl Extract<Vec<Vec<i64>>> for ProblemInput {
    fn extract(&self) -> Result<Vec<Vec<i64>>> {
        fn parse_with_sep(line: &str, sep: char) -> Vec<i64> {
            line.split(sep).map(|v| v.parse().unwrap()).collect()
        }
        fn parse_line(line: &str) -> Vec<i64> {
            if line.contains(',') {
                // parse the line as a comma separated list
                parse_with_sep(line, ',')
            } else if line.contains(' ') {
                // parse the line as a whitespace separated list
                parse_with_sep(line, ' ')
            } else if let Ok(parsed) = line.parse() {
                vec![parsed]
            } else {
                // potentially empty line
                vec![]
            }
        }

        Ok(self
            .lines
            .iter()
            .map(|line| parse_line(line.as_str()))
            .collect())
    }
}

impl Extract<Vec<i64>> for ProblemInput {
    fn extract(&self) -> Result<Vec<i64>> {
        let inner: Vec<Vec<i64>> = self.extract()?;

        Ok(inner.into_iter().flatten().collect())
    }
}

impl From<Vec<String>> for ProblemInput {
    fn from(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

impl From<Vec<&str>> for ProblemInput {
    fn from(lines: Vec<&str>) -> Self {
        Self::from(lines.into_iter().map(String::from).collect::<Vec<_>>())
    }
}

/// A trait representing a generic solution to an AoC problem.
// TODO: might want to be generic over return type
// or perhaps Box<dyn ToString> or something like that.
pub trait Solution: Send + Sync {
    fn part1(&self, lines: &ProblemInput) -> i64;
    fn part2(&self, lines: &ProblemInput) -> i64;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Extract<Vec<Point>> for ProblemInput {
    fn extract(&self) -> Result<Vec<Point>> {
        let mut points = Vec::with_capacity(self.lines.len());

        for line in self.lines.iter() {
            let mut split = line.split(", ");

            let part1 = &(split.next().unwrap())[3..];
            let part2 = &(split.next().unwrap())[2..];
            let part3 = split.next().unwrap();
            let part3 = &(part3)[2..(part3.len() - 1)];

            points.push(Point::new(
                part1.parse().unwrap(),
                part2.parse().unwrap(),
                part3.parse().unwrap(),
            ));
        }

        Ok(points)
    }
}
