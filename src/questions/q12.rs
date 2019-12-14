use crate::Point;
use crate::{Extract, ProblemInput, Solution};
use num::Integer;
use std::collections::HashSet;

fn energy(points: &[Point], velocities: &[Point]) -> i64 {
    points
        .iter()
        .zip(velocities.iter())
        .map(|(u, v)| (u.x.abs() + u.y.abs() + u.z.abs()) * (v.x.abs() + v.y.abs() + v.z.abs()))
        .sum::<i64>()
}

fn equalize(mut pos: Vec<i64>) -> i64 {
    let mut steps = 0;
    let mut vels = vec![0; pos.len()];
    let mut visited = HashSet::new();

    loop {
        for (index, u) in pos.iter().enumerate() {
            for (index2, v) in pos.iter().enumerate() {
                if index2 <= index {
                    continue;
                }
                let v = if u < v {
                    1
                } else if u > v {
                    -1
                } else {
                    0
                };
                vels[index] += v;
                vels[index2] -= v;
            }
        }

        // update positions
        for (index, v) in vels.iter().enumerate() {
            pos[index] += v;
        }

        let state = (pos.clone(), vels.clone());

        if visited.contains(&state) {
            break;
        }

        steps += 1;
        visited.insert(state);
    }

    steps
}

pub struct Q12;

impl Solution for Q12 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut points: Vec<Point> = lines.extract().unwrap();
        let mut velocity = vec![Point::new(0, 0, 0); points.len()];

        for _ in 0..1000 {
            // compute gravity for each object
            for (index, u) in points.iter().enumerate() {
                for v in points.iter() {
                    if u == v {
                        continue;
                    }

                    let velocity_x = if u.x < v.x {
                        1
                    } else if u.x > v.x {
                        -1
                    } else {
                        0
                    };
                    let velocity_y = if u.y < v.y {
                        1
                    } else if u.y > v.y {
                        -1
                    } else {
                        0
                    };
                    let velocity_z = if u.z < v.z {
                        1
                    } else if u.z > v.z {
                        -1
                    } else {
                        0
                    };

                    velocity[index].x += velocity_x;
                    velocity[index].y += velocity_y;
                    velocity[index].z += velocity_z;
                }
            }

            // add velocity to position
            for (index, v) in velocity.iter().enumerate() {
                points[index].x += v.x;
                points[index].y += v.y;
                points[index].z += v.z;
            }
        }

        energy(&points, &velocity)
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let points: Vec<Point> = lines.extract().unwrap();
        let f1 = equalize(points.iter().map(|p| p.x).collect());
        let f2 = equalize(points.iter().map(|p| p.y).collect());
        let f3 = equalize(points.iter().map(|p| p.z).collect());

        f1.lcm(&f2).lcm(&f3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q12 = Q12;
        assert_eq!(q12.part1(&load_problem_input(12)), 5_517);
    }

    #[test]
    fn test_part2_solution() {
        let q12 = Q12;
        assert_eq!(q12.part2(&load_problem_input(12)), 303_070_460_651_184);
    }
}
