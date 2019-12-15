use crate::{binary_search_by_key, Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::HashMap;

pub struct Q14;

impl Solution for Q14 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let oremap: OreMap = lines.extract().unwrap();

        // need at least 1 fuel
        let mut resources = Resources::new(1);
        resources.resolve(&oremap)
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let oremap: OreMap = lines.extract().unwrap();

        binary_search_by_key(1_000_000, 2_000_000, 1_000_000_000_000, |fuel| {
            Resources::new(fuel).resolve(&oremap)
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OreId(usize);

#[derive(Debug, Clone)]
pub struct OreMap {
    pub ores: HashMap<String, OreId>,
    pub transformation: HashMap<OreId, (i64, Vec<(i64, OreId)>)>,
}

impl OreMap {
    pub fn new() -> Self {
        let mut ores = HashMap::new();
        ores.insert(String::from("FUEL"), OreId(0));
        ores.insert(String::from("ORE"), OreId(1));

        Self {
            ores,
            transformation: HashMap::new(),
        }
    }

    pub fn ore(&self, id: OreId) -> String {
        for (key, &v) in self.ores.iter() {
            if v == id {
                return String::from(key);
            }
        }

        panic!();
    }

    pub fn find_or_insert(&mut self, ore: String) -> OreId {
        // Need to get the current length here to avoid borrow checker issues with using .entry()
        let current_length = self.ores.len();

        let entry = self.ores.entry(ore).or_insert(OreId(current_length));
        *entry
    }
}

impl Default for OreMap {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_part(oremap: &mut OreMap, part: &str) -> Vec<(i64, OreId)> {
    let mut res = Vec::new();

    for segment in part.split(", ") {
        let mut segment = segment.split(' ');
        let left = segment.next().unwrap();
        let right = segment.next().unwrap();

        res.push((
            left.parse().unwrap(),
            oremap.find_or_insert(String::from(right)),
        ));
    }

    res
}

impl Extract<OreMap> for ProblemInput {
    fn extract(&self) -> Result<OreMap> {
        let mut oremap = OreMap::new();

        self.lines.iter().for_each(|line| {
            let mut parts = line.split(" => ");

            let left = parts.next().unwrap();
            let right = parts.next().unwrap();

            let l = parse_part(&mut oremap, left);
            let r = parse_part(&mut oremap, right);

            let target_id = r[0].1;
            let target_quantity = r[0].0;

            oremap
                .transformation
                .insert(target_id, (target_quantity, l));
        });

        Ok(oremap)
    }
}

#[derive(Debug, Clone)]
pub struct Resources {
    pub have: HashMap<OreId, i64>,
}

impl Resources {
    pub fn new(fuel_quantity: i64) -> Self {
        let mut have = HashMap::new();
        have.insert(OreId(0), fuel_quantity);

        Self { have }
    }

    pub fn resolve(&mut self, map: &OreMap) -> i64 {
        while let Some((id, v)) = self.have.iter().find_map(|(&id, &v)| {
            if v > 0 && id.0 != 1 {
                Some((id, v))
            } else {
                None
            }
        }) {
            // remove this entry from the have
            let (m, sources) = map.transformation.get(&id).unwrap();
            let num_trades = ((v as f64) / (*m as f64)).ceil() as i64;
            *self.have.get_mut(&id).unwrap() -= *m * num_trades;

            for (quantity, source) in sources {
                *self.have.entry(*source).or_insert(0) += *quantity * num_trades;
            }
        }

        *self.have.get(&OreId(1)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q14 = Q14;
        assert_eq!(q14.part1(&load_problem_input(14)), 783_895);
    }

    #[test]
    fn test_part2_solution() {
        let q14 = Q14;
        assert_eq!(q14.part2(&load_problem_input(14)), 1_896_688);
    }
}
