use crate::grid::Direction;
use crate::grid::Position;
use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::Queue;
use crate::ic::ICPostAction;
use crate::{Extract, ProblemInput, Solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Q15;

impl Solution for Q15 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let interpreter: ICInterpreter = lines.extract().unwrap();
        let mut discoverer = Discoverer::new(interpreter);
        discoverer.discover();

        let oxygen = discoverer.oxygen.unwrap();
        discoverer.best_so_far.get(&oxygen).unwrap().len() as i64
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let interpreter: ICInterpreter = lines.extract().unwrap();
        let mut discoverer = Discoverer::new(interpreter);
        discoverer.discover();

        let oxygen_position = discoverer.oxygen.unwrap();

        let mut visited = HashSet::new();
        let mut oxygen_frontier = HashSet::new();
        oxygen_frontier.insert(oxygen_position);

        // hilarious off by 1 error
        let mut minutes = -1;

        while !oxygen_frontier.is_empty() {
            minutes += 1;

            // keep track of all the places the oxygen expands to in this minute
            let mut new_oxygen = HashSet::new();

            for position in oxygen_frontier.iter() {
                visited.insert(*position);

                for &direction in Direction::all().iter() {
                    let target_position = position.go(direction);

                    if discoverer.blocked.contains(&target_position)
                        || visited.contains(&target_position)
                    {
                        continue;
                    }
                    new_oxygen.insert(target_position);
                }
            }

            // update the frontier
            oxygen_frontier = new_oxygen;
        }

        minutes
    }
}

pub struct Discoverer {
    /// Keep track of our current position in case we need to reset
    pub current_position: Position,
    /// Keep track of the positions we still have to explore & the directions we can go from
    pub work: VecDeque<(Position, HashSet<Direction>)>,
    /// Keep track of blocked positions
    pub blocked: HashSet<Position>,
    /// Keep track of the quickest directions to get to each position
    pub best_so_far: HashMap<Position, Vec<Direction>>,
    /// Our IC interpreter
    pub interpreter: ICInterpreter,
    /// Location of the oxygen
    pub oxygen: Option<Position>,
}

impl Discoverer {
    pub fn new(interpreter: ICInterpreter) -> Self {
        let mut interpreter = interpreter;

        interpreter.postprocess(4, |_, fz: &mut ICPostAction| {
            // convert output finalization continue states to terminate
            if let ICPostAction::Continue = fz {
                *fz = ICPostAction::Terminate;
            };
        });

        let current_position = Position::new(0, 0);
        let mut work = VecDeque::new();
        work.push_front((current_position, Direction::all()));

        let mut best_so_far = HashMap::new();
        best_so_far.insert(current_position, vec![]);

        Self {
            current_position,
            work,
            blocked: HashSet::new(),
            best_so_far,
            interpreter,
            oxygen: None,
        }
    }

    /// Roam around, discover the map.
    pub fn discover(&mut self) {
        'work_loop: while let Some((position, candidate_directions)) = self.work.pop_front() {
            // move the discoverer to `position` if we aren't already there
            if self.current_position != position {
                self.move_to(position);
            }

            // keep track of candidate directions for the current position (in case we come back here)
            let mut future_candidation_directions = candidate_directions.clone();

            for &direction in candidate_directions.iter() {
                // We never want to explore this direction again, no matter what happens
                future_candidation_directions.remove(&direction);
                let candidate_position = position.go(direction);

                // we never want to explore to a position we know is blocked, or that we've been to before.
                // since we're doing a BST we're guaranteed to find the shortest path to each position the first time
                // we arrive there
                if self.blocked.contains(&candidate_position)
                    || self.best_so_far.contains_key(&candidate_position)
                {
                    continue;
                }

                // attempt to move in the selected direction
                self.interpreter
                    .run_with_inputs(vec![direction_to_int(direction)]);
                let output = self.interpreter.outputs.pop().unwrap();

                if output == 0 {
                    // we've reached a blocked tile - try another direction
                    self.blocked.insert(candidate_position);
                    continue;
                }

                // did we find the oxygen?
                if output == 2 {
                    self.oxygen = Some(candidate_position);
                }

                // otherwise update `self.best_so_far` for the new position
                let mut current_best_so_far = self.best_so_far.get(&position).unwrap().clone();
                current_best_so_far.push(direction);
                self.best_so_far
                    .insert(candidate_position, current_best_so_far);

                // put the old position at the back onto our work queue if there are still unexplored directions
                if !future_candidation_directions.is_empty() {
                    self.work
                        .push_back((position, future_candidation_directions));
                }

                // add the new position to the front of our queue
                self.work.push_front((candidate_position, Direction::all()));

                continue 'work_loop;
            }
        }
    }

    pub fn move_to(&mut self, position: Position) {
        self.interpreter.reset();

        for &directions in self.best_so_far.get(&position).unwrap() {
            self.interpreter
                .run_with_inputs(vec![direction_to_int(directions)]);
        }

        // clear outputs TODO encapsulate
        self.interpreter.outputs.outputs.clear();

        self.current_position = position;
    }
}

fn direction_to_int(dir: Direction) -> i64 {
    match dir {
        Direction::Up => 1,
        Direction::Down => 2,
        Direction::Left => 3,
        Direction::Right => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q15 = Q15;
        assert_eq!(q15.part1(&load_problem_input(15)), 208);
    }

    #[test]
    fn test_part2_solution() {
        let q15 = Q15;
        assert_eq!(q15.part2(&load_problem_input(15)), 306);
    }
}
