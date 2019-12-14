use crate::grid::Position;
use crate::{Extract, ProblemInput, Solution};
use anyhow::{Context, Result};
use num::Integer;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct AsteroidMap {
    pub asteroids: HashSet<Position>,
    pub height: usize,
    pub width: usize,
}

impl AsteroidMap {
    pub fn new(asteroids: HashSet<Position>, height: usize, width: usize) -> Self {
        Self {
            asteroids,
            height,
            width,
        }
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0
            && position.y >= 0
            && (position.x as usize) < self.width
            && (position.y as usize) < self.height
    }

    pub fn contains(&self, position: Position) -> bool {
        self.asteroids.contains(&position)
    }
}

impl Extract<AsteroidMap> for ProblemInput {
    fn extract(&self) -> Result<AsteroidMap> {
        let height = self.lines.len();
        let width = self
            .lines
            .get(0)
            .with_context(|| "got map with zero width")?
            .len();

        let asteroids = self
            .lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Position::new(x as i64, y as i64))
            })
            .flatten()
            .collect::<HashSet<_>>();

        Ok(AsteroidMap::new(asteroids, height, width))
    }
}

fn angle_map(map: &AsteroidMap) -> HashMap<Position, Vec<(Position, f64, i64)>> {
    // Build up a `HashMap` whose keys are asteroids, and whose values are vectors of triples:
    // `(other_asteroid, angle, distance)`.
    let mut by_angle = HashMap::new();
    for &asteroid in map.asteroids.iter() {
        let entry = by_angle.entry(asteroid).or_insert_with(Vec::new);

        for &other_asteroid in map.asteroids.iter() {
            if asteroid == other_asteroid {
                continue;
            }

            // compute the angle betwixt the asteroids
            let angle = ((other_asteroid.y - asteroid.y) as f64)
                .atan2((other_asteroid.x - asteroid.x) as f64)
                .to_degrees();

            // compute the distance betwixt the asteroids
            let distance =
                (other_asteroid.y - asteroid.y).pow(2) + (other_asteroid.x - asteroid.x).pow(2);

            entry.push((other_asteroid, angle, distance));
        }
    }

    // For each point, order its victims starting with an angle of -90.0 and going clockwise.
    // If two victims share an angle, the tie goes to the closer one.
    for (_, v) in by_angle.iter_mut() {
        v.sort_by_key(|(_, ang, dst)| {
            let of = OrderedFloat(*ang);

            (
                if of >= OrderedFloat(-90.0) { 0 } else { 1 },
                OrderedFloat(*ang),
                *dst,
            )
        });
    }

    by_angle
}

pub struct Q10;

impl Solution for Q10 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let map: AsteroidMap = lines.extract().unwrap();

        let n = map.asteroids.len();
        let mut blockers: HashMap<Position, HashSet<Position>> = HashMap::new();

        for &asteroid in map.asteroids.iter() {
            let curr_blockers = blockers.entry(asteroid).or_insert_with(HashSet::new);

            for &other_asteroid in map.asteroids.iter() {
                if asteroid == other_asteroid || curr_blockers.contains(&other_asteroid) {
                    continue;
                }

                let delta = other_asteroid - asteroid;
                let gcd = delta.x.gcd(&delta.y);
                let delta = Position::new(delta.x / gcd, delta.y / gcd);

                let mut current = other_asteroid + delta;
                while map.in_bounds(current) {
                    // check if the current position is an asteroid
                    if map.contains(current) {
                        // mark this as a blocker
                        curr_blockers.insert(current);
                    }
                    current = current + delta;
                }
            }
        }

        let counts = blockers
            .into_iter()
            .map(|(pos, blockers)| (pos, blockers.len()))
            .collect::<Vec<_>>()
            .into_iter()
            .min_by_key(|&(_, count)| count)
            .unwrap();

        (n - counts.1 - 1) as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let map: AsteroidMap = lines.extract().unwrap();
        let mut angle_map = angle_map(&map);

        // We only care about the point (11, 11).
        let ordered_points = angle_map
            .iter_mut()
            .find(|pos| pos.0.x == 11 && pos.0.y == 11)
            .unwrap()
            .1;

        let mut counter = 0;
        while !ordered_points.is_empty() {
            // Keep track of the last angle we saw
            let mut last_angle = OrderedFloat(-1000.0);
            let mut ix = 0;

            while ix < ordered_points.len() {
                let pt = ordered_points[ix];
                let of = OrderedFloat(pt.1);

                if of != last_angle {
                    last_angle = of;

                    // If we've found the 200th laser victim, we're done.
                    counter += 1;
                    if counter == 200 {
                        return pt.0.x * 100 + pt.0.y;
                    }

                    // Otherwise DESTROY this one.
                    ordered_points.remove(ix);
                } else {
                    // If we haven't found another angle, then we're looking at asteroids hidden behind other asteroids.
                    ix += 1;
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q10 = Q10;
        assert_eq!(q10.part1(&load_problem_input(10)), 221);
    }

    #[test]
    fn test_part2_solution() {
        let q10 = Q10;
        assert_eq!(q10.part2(&load_problem_input(10)), 806);
    }
}
