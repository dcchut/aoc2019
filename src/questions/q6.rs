use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct Q6;

impl Solution for Q6 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut orbit_graph: OrbitGraph = lines.extract().unwrap();

        (0..orbit_graph.size())
            .map(|index| orbit_graph.ancestors(index).len())
            .sum::<usize>() as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut orbit_graph: OrbitGraph = lines.extract().unwrap();

        // Find where "YOU" and "SAN" are
        let you_index = orbit_graph.insert("YOU");
        let san_index = orbit_graph.insert("SAN");

        // Find their parent nodes
        let you_parent_index = orbit_graph.lineage[&you_index];
        let san_parent_index = orbit_graph.lineage[&san_index];

        // Find all ancestors of you and san
        let you_ancestors = orbit_graph.ancestors(you_parent_index);
        let san_ancestors = orbit_graph.ancestors(san_parent_index);

        // Create a hashset of SAN-cestors for faster lookup
        let san_ancestors_set: HashSet<usize> = san_ancestors.iter().cloned().collect();

        // Find the first entry of you_ancestors that occurs in san_ancestors_set
        let (you_length, key) = you_ancestors
            .into_iter()
            .enumerate()
            .find(|(_, item)| san_ancestors_set.contains(item))
            .unwrap();
        // Then find the first time it occurs in san_ancestors
        let (san_length, _) = san_ancestors
            .into_iter()
            .enumerate()
            .find(|(_, item)| item == &key)
            .unwrap();

        // Since the vec is zero-indexed, we need to add 1 + 1 to get the number of ancestors
        (you_length + san_length + 2) as i64
    }
}

#[derive(Debug, Clone)]
struct OrbitGraph {
    names: Vec<String>,
    lineage: HashMap<usize, usize>,
    ancestors: HashMap<usize, Vec<usize>>,
}

impl OrbitGraph {
    /// Inserts an input string `s` into the `OrbitGraph`, returning the index at which `s`
    /// was inserted.  If `s` is already contained in the `OrbitGraph`, then this function instead
    /// returns the index of `s`.
    pub fn insert<T>(&mut self, s: T) -> usize
    where
        String: From<T>,
    {
        let s = String::from(s);

        // check if this string is already in our list of names
        for (index, name) in self.names.iter().enumerate() {
            if name == &s {
                return index;
            }
        }

        // otherwise insert it at the end and return the inserted index
        self.names.push(s);
        self.names.len() - 1
    }

    /// Records a parent -> child relationship in the `OrbitGraph`.
    pub fn add_relationship(&mut self, parent: usize, child: usize) {
        self.lineage.insert(child, parent);
    }

    /// Returns the number of nodes in the `OrbitGraph`.
    pub fn size(&self) -> usize {
        self.names.len()
    }

    /// Creates a new (empty) `OrbitGraph` instance.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            lineage: HashMap::new(),
            ancestors: HashMap::new(),
        }
    }

    /// Returns a vector containing all ancestors of a given node.
    pub fn ancestors(&mut self, node: usize) -> Vec<usize> {
        // If we've already computed how many ancestors this node has, return then
        if let Some(ancestors) = self.ancestors.get(&node) {
            return ancestors.clone();
        }

        // Otherwise, compute recursively
        let ancestors = if let Some(parent) = self.lineage.get(&node).cloned() {
            let mut parental_ancestors = vec![parent];
            parental_ancestors.extend(self.ancestors(parent));

            parental_ancestors
        } else {
            vec![]
        };

        self.ancestors.insert(node, ancestors.clone());
        ancestors
    }
}

impl Extract<OrbitGraph> for ProblemInput {
    fn extract(&self) -> Result<OrbitGraph> {
        let mut orbit_graph = OrbitGraph::new();

        // compute the lineage from the input
        for line in self.lines.iter() {
            // Split line into two parts: ABC)DE => (ABC, DE)
            let (l, r) = {
                let mut rb = line.split(')');
                (rb.next().unwrap(), rb.next().unwrap())
            };

            let left_index = orbit_graph.insert(l);
            let right_index = orbit_graph.insert(r);

            // record the child -> parent relationship
            orbit_graph.add_relationship(left_index, right_index);
        }

        Ok(orbit_graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_orbit_map() {
        let input = ProblemInput::from(vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]);
        let q6 = Q6 {};

        assert_eq!(q6.part1(&input), 42);

        let input = ProblemInput::from(vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]);

        assert_eq!(q6.part2(&input), 4);
    }

    #[test]
    fn test_part1_solution() {
        let q6 = Q6;
        assert_eq!(q6.part1(&load_problem_input(6)), 387_356);
    }

    #[test]
    fn test_part2_solution() {
        let q6 = Q6;
        assert_eq!(q6.part2(&load_problem_input(6)), 532)
    }
}
