use crate::grid::{Direction, Position};
use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use pathfinding::prelude::dijkstra_all;
use rayon::prelude::*;
use std::cmp::min;
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
    pub key_to_door: HashMap<char, Position>,
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
        let mut key_to_door = HashMap::new();

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
                        key_to_door.insert(c.to_ascii_lowercase(), pos);
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
            key_to_door,
        })
    }
}

fn shortest_paths(
    start: Position,
    ends: &HashSet<Position>,
    obstacles: &HashSet<Position>,
) -> HashMap<Position, i64> {
    use pathfinding::prelude::absdiff;
    use pathfinding::prelude::astar_bag;

    let result = dijkstra_all(&start, |pos| {
        let mut succ = Vec::with_capacity(4);

        for direction in Direction::all() {
            let tar = pos.go(direction);

            if !obstacles.contains(&tar) {
                succ.push((tar, 1));
            }
        }

        succ
    });

    result.into_iter().map(|(k, v)| (k, v.1 as i64)).collect()
}

fn shortest_path(start: Position, end: Position, obstacles: &HashSet<Position>) -> Option<usize> {
    use pathfinding::prelude::{absdiff, astar};

    let result = astar(
        &start,
        |pos| {
            let mut succ = vec![
                pos.go(Direction::Left),
                pos.go(Direction::Right),
                pos.go(Direction::Up),
                pos.go(Direction::Down),
            ];

            succ.into_iter()
                .filter(|pos| !obstacles.contains(pos))
                .map(|pos| (pos, 1))
                .collect::<Vec<_>>()
        },
        |pos| absdiff(pos.x, end.x) + absdiff(pos.y, end.y),
        |pos| *pos == end,
    );

    result.map(|pos| pos.0.len())
}

enum SolveResult {
    Steps(i64),
    States(Vec<(Position, usize, HashSet<Position>, HashSet<Position>)>),
}

impl Solution for Q18 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let dm: DungeonMap = lines.extract().unwrap();

        let mut state = VecDeque::new();

        let mut places_to_visit: HashSet<_> = dm
            .interaction
            .iter()
            .filter_map(|(position, interaction)| {
                if let DungeonInteractive::Key(c) = interaction {
                    Some(*position)
                } else {
                    None
                }
            })
            .collect();

        let mut obstacles = dm.obstacles.clone();
        // add all of the doors to our obstacles list
        dm.interaction
            .iter()
            .filter_map(|(pos, int)| {
                if let DungeonInteractive::Door(_) = int {
                    Some(*pos)
                } else {
                    None
                }
            })
            .for_each(|pos| {
                obstacles.insert(pos);
            });

        state.push_front((dm.entrance, 0, obstacles, places_to_visit));

        let mut best_steps = 13_687;

        while let Some((position, steps, obstacles, places_to_visit)) = state.pop_front() {
            if places_to_visit.is_empty() || steps as i64 >= best_steps {
                best_steps = min(best_steps, steps);
                continue;
            }

            let shortest_paths = shortest_paths(position, &places_to_visit, &obstacles);

            for (place, path_length) in shortest_paths {
                let mut new_entry = (
                    place,
                    steps + path_length as i64,
                    obstacles.clone(),
                    places_to_visit.clone(),
                );
                new_entry.3.remove(&place);

                // if this was a key, unlock the corresponding door for use
                if let Some(DungeonInteractive::Key(c)) = dm.interaction.get(&place) {
                    if let Some(door) = dm.key_to_door.get(&c) {
                        new_entry.2.remove(door);
                        new_entry.3.insert(*door);
                    }
                }

                state.push_front(new_entry);
            }

            //            let (position, steps, obstacles, places_to_visit) = state.pop_front().unwrap();
            //
            //            if places_to_visit.is_empty() || steps >= best_steps {
            //                if steps < best_steps {
            //                    dbg!(steps);
            //                }
            //                best_steps = min(best_steps, steps);
            //                continue;
            //            }
            //
            //            for place in places_to_visit.iter().cloned() {
            //                if let Some(path_length) = shortest_path(position, place, &obstacles) {
            //                    // move to this place!
            //                    let mut new_entry = (place, steps + path_length - 1, obstacles.clone(), places_to_visit.clone());
            //                    new_entry.3.remove(&place);
            //
            //                    // if this was a key, unlock the corresponding door for use
            //                    if let Some(DungeonInteractive::Key(c)) = dm.interaction.get(&place) {
            //                        if let Some(door) = dm.key_to_door.get(&c) {
            //                            new_entry.2.remove(door);
            //                            new_entry.3.insert(*door);
            //                        }
            //                    }
            //
            //                    state.push_front(new_entry);
            //                }
            //            }
            // choose
            //state.into_par_iter().map(|v| )
        }

        best_steps as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        0
    }
}
