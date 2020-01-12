use crate::grid::{Direction, Position};
use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Occupier {
    Bug,
    Empty,
}

#[derive(Clone, Debug)]
pub struct EcoGrid {
    pub occ: HashMap<Position, Occupier>,
    pub serial: Vec<Occupier>,
    //    pub occ : Vec<Vec<Occupier>>,
}

impl EcoGrid {
    //    pub fn serialize(&self) -> Vec<Occupier> {
    //        let mut occupiers = Vec::with_capacity(25);
    //        for y in 0..5 {
    //            for x in 0.. 5{
    //                occupiers.push(*self.occ.get(&Position::new(x,y)).unwrap());
    //            }
    //        }
    //
    //        occupiers
    //    }

    pub fn print(&self) {
        for y in 0..5 {
            for x in 0..5 {
                let occupier = *self.occ.get(&Position::new(x, y)).unwrap();

                print!(
                    "{}",
                    match occupier {
                        Occupier::Empty => '.',
                        Occupier::Bug => '#',
                    }
                );
            }
            println!();
        }
        println!();
    }

    pub fn get(&self, position: Position) -> Option<Occupier> {
        self.occ.get(&position).cloned()
    }

    pub fn adjacent_bugs(&self, position: Position) -> i64 {
        let mut adjacent_bugs = 0;

        // get all things adjacent to the given square
        for direction in Direction::all() {
            let target_position = position.go(direction);

            if let Some(Occupier::Bug) = self.get(target_position) {
                adjacent_bugs += 1;
            }
        }

        adjacent_bugs
    }

    pub fn mutate(&mut self) {
        let mut occ = self.occ.clone();

        // iterate over the grid
        for y in 0..5 {
            for x in 0..5 {
                let position = Position::new(x, y);
                let adjacent_bugs = self.adjacent_bugs(position);
                let c = (x + y * 5) as usize;

                if let Some(occupier) = self.occ.get(&position) {
                    match occupier {
                        Occupier::Empty => {
                            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                                // an empty space becomes infested if exactly one or two bugs are adjacent to it
                                occ.insert(position, Occupier::Bug);
                                self.serial[c] = Occupier::Bug;
                            }
                        }
                        Occupier::Bug => {
                            if adjacent_bugs != 1 {
                                occ.insert(position, Occupier::Empty);
                                self.serial[c] = Occupier::Empty;
                            }
                        }
                    }
                }
            }
        }

        self.occ = occ;
    }
}

impl Extract<EcoGrid> for ProblemInput {
    fn extract(&self) -> Result<EcoGrid> {
        let mut occ = HashMap::new();
        let mut serial = vec![Occupier::Empty; 25];

        for (y, line) in self.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let position = Position::new(x as i64, y as i64);
                let c = y * 5 + x;

                let occupier = match ch {
                    '#' => Occupier::Bug,
                    '.' => Occupier::Empty,
                    _ => panic!("invalid occupancy"),
                };

                occ.insert(position, occupier);
                serial[c] = occupier;
            }
        }

        Ok(EcoGrid { occ, serial })
    }
}

pub struct Q24;

impl Solution for Q24 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut eco_grid: EcoGrid = lines.extract().unwrap();
        let mut seen = HashSet::new();

        while !seen.contains(&eco_grid.serial) {
            seen.insert(eco_grid.serial.clone());
            eco_grid.mutate();
        }

        let mut rating = 0;
        for (index, occupier) in eco_grid.serial.iter().enumerate() {
            if let Occupier::Bug = occupier {
                rating += (2_i64).pow(index as u32);
            }
        }

        rating
    }

    fn part2(&self, _lines: &ProblemInput) -> i64 {
        0
    }
}
