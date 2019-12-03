use aoc2019::{Extract, ProblemInput, Solution};

pub struct Q3;

use std::collections::{HashSet, HashMap};

impl Solution for Q3 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let input = lines.lines.clone();

        let path1 = input[0].split(",").collect::<Vec<_>>();
        let path2 = input[1].split(",").collect::<Vec<_>>();

        let mut current_position = (0,0);
        let mut visited = HashSet::new();

        let mut visited2 = HashSet::new();

        for part in path1 {
            let chars = part.chars().collect::<Vec<_>>();

            let direction = chars[0];

            if direction == '\n' {
                continue;
            }

            let distance : i64 = (&part[1..]).parse().unwrap();

            if direction == 'L' {
                for x in current_position.0 - distance ..= current_position.0 {
                    visited.insert((x, current_position.1));
                }
                current_position = (current_position.0 - distance, current_position.1);
            } else if direction == 'R' {
                for x in current_position.0 ..= current_position.0 + distance {
                    visited.insert((x, current_position.1));
                }
                current_position = (current_position.0 + distance, current_position.1);
            } else if direction == 'U' {
                for y in current_position.1 ..= current_position.1 + distance {
                    visited.insert((current_position.0, y));
                }
                current_position = (current_position.0, current_position.1 + distance);
            } else {
                for y in current_position.1 - distance ..= current_position.1 {
                    visited.insert((current_position.0, y));
                }
                current_position = (current_position.0, current_position.1 - distance);
            }
            visited.insert(current_position);
        }

        current_position = (0,0);
        for part in path2 {
            let chars = part.chars().collect::<Vec<_>>();

            let direction = chars[0];

            if direction == '\n' {
                continue;
            }

            let distance : i64 = (&part[1..]).parse().unwrap();

            if direction == 'L' {
                for x in current_position.0 - distance ..= current_position.0 {
                    visited2.insert((x, current_position.1));
                }
                current_position = (current_position.0 - distance, current_position.1);
            } else if direction == 'R' {
                for x in current_position.0 ..= current_position.0 + distance {
                    visited2.insert((x, current_position.1));
                }
                current_position = (current_position.0 + distance, current_position.1);
            } else if direction == 'U' {
                for y in current_position.1 ..= current_position.1 + distance {
                    visited2.insert((current_position.0, y));
                }
                current_position = (current_position.0, current_position.1 + distance);
            } else {
                for y in current_position.1 - distance ..= current_position.1 {
                    visited2.insert((current_position.0, y));
                }
                current_position = (current_position.0, current_position.1 - distance);
            }
            visited2.insert(current_position);
        }

        let mut best_distance = 1000000000000;

        for point in visited.intersection(&visited2) {
            let distance = abs(point.0) + abs(point.1);

            if distance > 0 && distance < best_distance {
                best_distance = distance;
            }
        }

        best_distance
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let input = lines.lines.clone();

        let path1 = input[0].split(",").collect::<Vec<_>>();
        let path2 = input[1].split(",").collect::<Vec<_>>();

        let mut current_position = (0,0);
        let mut visited = HashSet::new();

        let mut visited2 = HashSet::new();
        let mut steps = 1;
        let mut visited_time = HashMap::new();
        let mut visited_time2 = HashMap::new();

        for part in path1 {
            let chars = part.chars().collect::<Vec<_>>();

            let direction = chars[0];

            if direction == '\n' {
                continue;
            }

            let distance : i64 = (&part[1..]).parse().unwrap();

            if direction == 'L' {
                for delta in 1 ..= distance {
                    let x = current_position.0 - delta;
                    
                    visited.insert((x, current_position.1));
                    if !visited_time.contains_key(&(x, current_position.1)) {
                        visited_time.insert((x, current_position.1), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0 - distance, current_position.1);
            } else if direction == 'R' {
                for delta in 1 ..= distance {
                    let x = current_position.0 + delta;
                    
                    visited.insert((x, current_position.1));
                    if !visited_time.contains_key(&(x, current_position.1)) {
                        visited_time.insert((x, current_position.1), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0 + distance, current_position.1);
            } else if direction == 'U' {
                for delta in 1 ..= distance {
                    let y = current_position.1 + delta;
                    
                    visited.insert((current_position.0, y));
                    if !visited_time.contains_key(&(current_position.0, y)) {
                        visited_time.insert((current_position.0, y), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0, current_position.1 + distance);
            } else {
                for delta in 1 ..= distance {
                    let y = current_position.0 - delta;
                    
                    visited.insert((current_position.0, y));
                    if !visited_time.contains_key(&(current_position.0, y)) {
                        visited_time.insert((current_position.0, y), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0, current_position.1 - distance);
            }
            visited.insert(current_position);
        }

        steps = 1;
        current_position = (0,0);
        for part in path2 {
            let chars = part.chars().collect::<Vec<_>>();

            let direction = chars[0];

            if direction == '\n' {
                continue;
            }

            let distance : i64 = (&part[1..]).parse().unwrap();

            if direction == 'L' {
                for delta in 1 ..= distance {
                    let x = current_position.0 - delta;
                    
                    visited2.insert((x, current_position.1));
                    if !visited_time2.contains_key(&(x, current_position.1)) {
                        visited_time2.insert((x, current_position.1), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0 - distance, current_position.1);
            } else if direction == 'R' {
                for delta in 1 ..= distance {
                    let x = current_position.0 + delta;
                    
                    visited2.insert((x, current_position.1));
                    if !visited_time2.contains_key(&(x, current_position.1)) {
                        visited_time2.insert((x, current_position.1), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0 + distance, current_position.1);
            } else if direction == 'U' {
                for delta in 1 ..= distance {
                    let y = current_position.1 + delta;
                    
                    visited2.insert((current_position.0, y));
                    if !visited_time2.contains_key(&(current_position.0, y)) {
                        visited_time2.insert((current_position.0, y), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0, current_position.1 + distance);
            } else {
                for delta in 1 ..= distance {
                    let y = current_position.1 - delta;
                    
                    visited2.insert((current_position.0, y));
                    if !visited_time2.contains_key(&(current_position.0, y)) {
                        visited_time2.insert((current_position.0, y), steps);
                    }
                    steps += 1;
                }
                current_position = (current_position.0, current_position.1 - distance);
            }
            visited2.insert(current_position);
        }

        let mut best_distance = 1000000000000;

        for point in visited.intersection(&visited2) {
            let distance = visited_time[point] + visited_time2[point];

            if distance > 0 && distance < best_distance {
                best_distance = distance;
            }
        }

        best_distance
    }
}

fn abs(x : i64) -> i64 {
    if x < 0 {
        -x
    } else {
        x
    }
}