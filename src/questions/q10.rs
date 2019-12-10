use anyhow::{Context, Result};
use aoc2019::grid::Position;
use aoc2019::{Extract, ProblemInput, Solution};
use num::Integer;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Map {
    pub asteroids: HashSet<Position>,
    pub height: usize,
    pub width: usize,
}

impl Map {
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

impl Extract<Map> for ProblemInput {
    fn extract(&self) -> Result<Map> {
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

        Ok(Map::new(asteroids, height, width))
    }
}

pub struct Q10;

impl Solution for Q10 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let map: Map = lines.extract().unwrap();

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

        dbg!(counts.0);

        (n - counts.1 - 1) as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let map: Map = lines.extract().unwrap();

        dbg!(map.asteroids.len());

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

                let distance =
                    (other_asteroid.y - asteroid.y).pow(2) + (other_asteroid.x - asteroid.x).pow(2); //

                let index = entry
                    .binary_search_by_key(&OrderedFloat(angle), |(_, ang, _)| OrderedFloat(*ang));

                entry.insert(
                    match index {
                        Ok(index) => index,
                        Err(index) => index,
                    },
                    (other_asteroid, angle, distance),
                );
            }
        }

        // Now collect each of these vectors based on min distance
        // let mut col = Vec::new();

        for (_, kaleidoscope) in by_angle.iter_mut() {
            kaleidoscope.sort_by_key(|(_, ang, dst)| {
                let of = OrderedFloat(*ang);

                (
                    if of >= OrderedFloat(-90.0) { 0 } else { 1 },
                    OrderedFloat(*ang),
                    *dst,
                )
            });
        }

        let mut desire = None;

        for pos in by_angle.keys() {
            if pos.x == 11 && pos.y == 11 {
                desire = Some(pos);
            }
        }

        let kal = by_angle
            .get_mut(unsafe { (desire.unwrap() as *const Position).as_ref().unwrap() })
            .unwrap();

        let mut z = 0;
        while !kal.is_empty() {
            let mut last_angle = OrderedFloat(-1000.0);
            let mut ix = 0;

            while ix < kal.len() {
                if OrderedFloat(kal[ix].1) != last_angle {
                    // remove this index from ka?
                    last_angle = OrderedFloat(kal[ix].1);

                    z += 1;
                    if z == 200 {
                        return kal[ix].0.x * 100 + kal[ix].0.y;
                    }
                    kal.remove(ix);
                } else {
                    ix += 1;
                }
            }
        }

        0
    }
}
