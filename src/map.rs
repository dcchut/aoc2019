// should have a notion of "keys", "doors", "teleports"

use crate::grid::{Direction, Position};
use crate::{Extract, ProblemInput};
use anyhow::Result;
use pathfinding::directed::dijkstra::{build_path, dijkstra_all};
use std::collections::{HashMap, HashSet};

//#[derive(Copy, Clone, PartialEq, Eq, Hash)]
//pub enum Places {
//    Empty,
//    Key(i64),
//    Door(i64),
//    TeleportSender(i64),
//    TeleportReceiver(i64),
//}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key(i64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Door(i64);

impl Door {
    pub fn new(content: i64) -> Self {
        Self(content)
    }

    pub fn from_key(key: Key) -> Self {
        Self::new(key.0)
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub keys: HashMap<Position, Key>,
    pub doors: HashMap<Position, Door>,
    pub key_to_door: HashMap<Key, Position>,
    pub blocked: HashSet<Position>,
    pub width: usize,
    pub height: usize,
    pub entrance: Position,
}

impl Map {
    pub fn shortest_paths(&self) -> HashMap<(Position, Position), (i64, HashSet<Door>)> {
        let mut shortest_paths = HashMap::new();
        let mut positions = vec![self.entrance];
        positions.extend(self.keys.keys());

        // compute the shortest paths from each key (+ the entrance) to each other key (+ the entrance)
        let spaths: HashMap<Position, HashMap<Position, Vec<Position>>> = positions
            .iter()
            .map(|&position| (position, self.shortest_paths_from_position(position)))
            .collect();

        for &pos1 in positions.iter() {
            for &pos2 in positions.iter() {
                if pos1 == pos2 {
                    continue;
                }

                let inner = spaths.get(&pos1).unwrap();
                let path = inner.get(&pos2).unwrap();

                // which doors are on this path?
                let doors = path
                    .into_iter()
                    .filter_map(|position| self.doors.get(position))
                    .cloned()
                    .collect();

                shortest_paths.insert((pos1, pos2), ((path.len() as i64) - 1, doors));
            }
        }

        shortest_paths
    }

    pub fn shortest_paths_from_position(
        &self,
        position: Position,
    ) -> HashMap<Position, Vec<Position>> {
        // Compute a hashmap from position to length of shortest path from `position` to that position
        let djr = dijkstra_all(&position, |current_position| {
            Direction::all()
                .into_iter()
                .filter_map(|direction| {
                    let successor = current_position.go(direction);

                    if !self.blocked.contains(&successor) {
                        Some((successor, 1_i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        });

        let mut result = HashMap::new();

        for (target_position, (_, steps)) in djr.iter() {
            let path = build_path(target_position, &djr);

            assert_eq!(path.len() as i64, *steps + 1);
            result.insert(*target_position, path);
        }

        result
    }

    fn reachable_nodes(&self, position: Position) -> HashMap<Position, (Position, i64)> {
        dijkstra_all(&position, |current_position| {
            let mut successors = Vec::new();

            for direction in Direction::all() {
                let successor = current_position.go(direction);

                if !self.doors.contains_key(&successor) && !self.blocked.contains(&successor) {
                    // can move to this position!
                    successors.push((successor, 1_i64));
                }
            }
            successors
        })
    }

    /// Returns a vec containing the position of all keys accessible from the given position (along with
    /// the length of a shortest path to that key).
    pub fn reachable_keys(&self, position: Position) -> Vec<(Position, i64)> {
        self.reachable_nodes(position)
            .into_iter()
            .filter(|(position, _)| self.keys.contains_key(position))
            .map(|(position, (_, score))| (position, score))
            .collect()
    }
}

impl Extract<Map> for ProblemInput {
    fn extract(&self) -> Result<Map> {
        assert!(!self.lines.is_empty());

        let width = self.lines[0].len();
        assert_ne!(width, 0);

        let height = self.lines.len();
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let mut blocked = HashSet::new();
        let mut key_to_door = HashMap::new();
        let mut entrance = None;

        for (y, line) in self.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let position = Position::new(x as i64, y as i64);

                if ch == '#' {
                    // blocked
                    blocked.insert(position);
                } else if ch == '@' {
                    entrance = Some(position);
                } else if ch == '.' {
                    // empty position
                } else if ch.is_ascii_lowercase() {
                    // key
                    let key = Key(ch as u8 as i64);
                    keys.insert(position, key);
                } else if ch.is_ascii_uppercase() {
                    let ch = ch.to_ascii_lowercase();
                    // door
                    let key = Key(ch as u8 as i64);
                    let door = Door(ch as u8 as i64);
                    doors.insert(position, door);
                    key_to_door.insert(key, position);
                }
            }
        }

        Ok(Map {
            keys,
            doors,
            key_to_door,
            blocked,
            width,
            height,
            entrance: entrance.unwrap(),
        })
    }
}
