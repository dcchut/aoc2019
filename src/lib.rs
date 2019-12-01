/// `FromLine` indicates that a type could (potentially) be created from a string slice.
/// Similar in spirit to the `FromStr` standard library trait.
pub trait FromLine
where
    Self: Sized,
{
    fn from_line(line: &str) -> Option<Self>;
}

// TODO: write a macro to do these dumb impls for me
// TODO: figure out what other impls might be useful

impl FromLine for i32 {
    fn from_line(line: &str) -> Option<Self> {
        line.parse().ok()
    }
}

impl FromLine for i64 {
    fn from_line(line: &str) -> Option<Self> {
        line.parse().ok()
    }
}

impl FromLine for u32 {
    fn from_line(line: &str) -> Option<Self> {
        line.parse().ok()
    }
}

impl FromLine for u64 {
    fn from_line(line: &str) -> Option<Self> {
        line.parse().ok()
    }
}

impl FromLine for usize {
    fn from_line(line: &str) -> Option<Self> {
        line.parse().ok()
    }
}

// Blanket impl for building up a vector of things based on a line.
// We assume in this case that entries are whitespace-separated.
impl<T> FromLine for Vec<T>
where
    T: FromLine,
{
    fn from_line(line: &str) -> Option<Self> {
        Some(
            line.split_whitespace()
                .filter_map(|s| T::from_line(s))
                .collect(),
        )
    }
}

/// Our main conversion trait.  
// TODO: instead of using &[&str] use something like &ProblemInput.
pub trait FromLines {
    fn from_lines(lines: &[&str]) -> Self;
}

// Blanket impl for anything implementing FromLine
impl<L> FromLines for Vec<L>
where
    L: FromLine,
{
    fn from_lines(lines: &[&str]) -> Self {
        lines
            .iter()
            .filter_map(|line| L::from_line(*line))
            .collect()
    }
}

/// A trait representing a generic solution to an AoC problem.
// TODO: might want to be generic over return type
// or perhaps Box<dyn ToString> or something like that.
pub trait Solution {
    fn part1(&self, lines: &[&str]) -> i64;
    fn part2(&self, lines: &[&str]) -> i64;
}
