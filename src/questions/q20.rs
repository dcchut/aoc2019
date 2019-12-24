use crate::grid::Position;
use crate::{Extract, ProblemInput, Solution};
use anyhow::Result;
use std::collections::HashMap;

pub struct PortalMap {
    width: i64,
    height: i64,
    start: Position,
    end: Position,
    portals: HashMap<String, (Position, Position)>,
}

fn get_square(cmap: &[Vec<char>], x: i64, y: i64) -> char {
    if x >= 0 && y >= 0 && y < cmap.len() as i64 && x < cmap[0].len() as i64 {
        cmap[(y as usize)][(x as usize)]
    } else {
        '_'
    }
}

//
//fn find_portals(cmap: &[Vec<char>]) -> HashMap<String, (Position, Position)> {
//    for y in 0..cmap.len() {
//        for x in 0..cmap[y].len() {
//
//        }
//    }
//    // search far and wide, attempting to find the portal
//}

impl Extract<PortalMap> for ProblemInput {
    fn extract(&self) -> Result<PortalMap> {
        let cmap = self
            .lines
            .iter()
            .map(|v| v.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = cmap[0].len() as i64 - 4;
        let height = cmap.len() as i64 - 4;

        let mut walls = Vec::new();
        let mut letters = HashMap::new();

        for y in 0..cmap.len() as i64 {
            for x in 0..cmap[0].len() as i64 {
                let c = cmap[y as usize][x as usize];
                //get_square(&cmap, x, y);

                match c {
                    '#' => {
                        walls.push(Position::new(x, y));
                    },
                    '.' | ' ' | '\n' | '_' => {},
                    z => {
                        dbg!(z);
                        // otherwise mark down the letter at this position
                        letters.insert(Position::new(x, y), z);
                    }
                }
            }
        }

        dbg!(letters.len());

        // find the start and end positions

        // now identify

        // first, discover the width
        panic!()
    }
}

pub struct Q20;

impl Solution for Q20 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let pmap : PortalMap = lines.extract().unwrap();
        0
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        0
    }
}
