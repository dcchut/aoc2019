use crate::grid::{Direction, Position};
use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Q18;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DungeonInteractive {
    Key(char),
    Door(char),
}

#[derive(Debug, Clone)]
struct DungeonMap {
    pub width: i64,
    pub height: i64,
    pub doors: i64,
    pub obstacles: HashSet<Position>,
    pub entrance: Position,
    pub interaction: HashMap<Position, DungeonInteractive>,
}

impl Extract<DungeonMap> for ProblemInput {
    fn extract(&self) -> Result<DungeonMap> {
        let width = self.lines[0].len() as i64;
        let height = self.lines.len() as i64;
        let line_chars = self
            .lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut obstacles = HashSet::new();
        let mut interaction = HashMap::new();
        let mut entrance = None;
        let mut doors = 0;

        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x, y);
                let c = line_chars[y as usize][x as usize];

                if c == '#' {
                    obstacles.insert(pos);
                } else if c == '@' {
                    entrance = Some(pos);
                } else if c != '.' {
                    if c.is_ascii_lowercase() {
                        // key
                        interaction.insert(pos, DungeonInteractive::Key(c));
                    } else {
                        // door
                        doors += 1;
                        interaction.insert(pos, DungeonInteractive::Door(c));
                    }
                }
            }
        }

        Ok(DungeonMap {
            width,
            height,
            doors,
            obstacles,
            interaction,
            entrance: entrance.unwrap(),
        })
    }
}

impl Solution for Q18 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let dm: DungeonMap = lines.extract().unwrap();

        // do a BFS starting at the entrance
        let mut state = VecDeque::new();
        state.push_front((
            dm.entrance,
            0,
            HashSet::new(),
            HashSet::new(),
            Position::new(-999, -999),
        )); // current position, acquired keys

        while let Some((position, steps, visited, keys, last_position)) = state.pop_front() {
            // are we done
            if visited.len() == dm.doors as usize {
                return steps;
            }

            // from this position, try to go in every direction?
            for direction in Direction::all() {
                let new_position = position.go(direction);

                // wall
                if dm.obstacles.contains(&new_position) {
                    continue;
                }

                // interactive thing?
                if let Some(interaction) = dm.interaction.get(&new_position) {
                    if let DungeonInteractive::Key(c) = interaction {
                        // yay!
                        let mut new_entry = (
                            new_position,
                            steps + 1,
                            visited.clone(),
                            keys.clone(),
                            position,
                        );
                        new_entry.3.insert(*c);

                        state.push_back(new_entry);
                    } else if let DungeonInteractive::Door(c) = interaction {
                        // do we have the key for this door?
                        if keys.contains(&c.to_ascii_lowercase()) {
                            // we may go here
                            let mut new_entry = (
                                new_position,
                                steps + 1,
                                visited.clone(),
                                keys.clone(),
                                position,
                            );
                            new_entry.2.insert(*c);

                            state.push_back(new_entry);
                        }
                    }
                } else if new_position != last_position {
                    // plain ol' movement
                    let new_entry = (
                        new_position,
                        steps + 1,
                        visited.clone(),
                        keys.clone(),
                        position,
                    );
                    state.push_back(new_entry);
                }
            }
        }

        0
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        0
    }
}
