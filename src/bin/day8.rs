use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("./day8.txt");

#[derive(Debug)]
enum Cell {
    Antinode,
    Antenna(char),
    Empty,
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    #[inline]
    fn is_valid_coord(&self, p: (i64, i64)) -> bool {
        0 <= p.0 && p.0 < self.cells.len() as i64 &&
        0 <= p.1 && p.1 < self.cells[0].len() as i64
    }

    fn antinodes1(
        &self,
        p: &(usize, usize),
        q: &(usize, usize)
    ) -> Vec<(usize, usize)> {
        let r = (
            (2 * (p.0 as i64) - (q.0 as i64)),
            (2 * (p.1 as i64) - (q.1 as i64)),
        );

        let s = (
            (2 * (q.0 as i64) - (p.0 as i64)),
            (2 * (q.1 as i64) - (p.1 as i64)),
        );

        let antinodes = vec![r, s].into_iter()
            .filter(|&p| self.is_valid_coord(p))
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect::<Vec<_>>();

        antinodes
    }

    fn antinodes_infinity(
        &self,
        p: &(usize, usize),
        q: &(usize, usize)
    ) -> Vec<(usize, usize)> {
        let p = (p.0 as i64, p.1 as i64);
        let q = (q.0 as i64, q.1 as i64);

        let left = (0..self.cells.len() as i64)
            .map(|k| (p.0 + k * (p.0 - q.0), p.1 + k * (p.1 - q.1)))
            .take_while(|&p| self.is_valid_coord(p))
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect::<Vec<_>>();

        let right = (0..self.cells.len() as i64)
            .map(|k| (q.0 + k * (q.0 - p.0), q.1 + k * (q.1 - p.1)))
            .take_while(|&p| self.is_valid_coord(p))
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect::<Vec<_>>();

        let mut res = left;
        res.extend(right);
        res
    }

    fn find_unique_antinodes<F>(
        &self,
        find_antinodes_for: F,
    ) -> HashSet<(usize, usize)>
    where
        F: Fn(&Self, &(usize, usize), &(usize, usize)) -> Vec<(usize, usize)>
    {
        let mut antinodes = HashSet::new();

        for (_, locations) in &self.antennas {
            locations.iter().combinations(2).for_each(|pair| {
                antinodes.extend(find_antinodes_for(self, pair[0], pair[1]));
            });
        }

        antinodes
    }
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut antennas = HashMap::new();

        let cells = value.lines().enumerate()
                .map(|(i, line)| {
                    line.chars().enumerate().map(|(j, c)|
                        if c == '.' {
                            Cell::Empty 
                        } else {
                            antennas.entry(c)
                                .and_modify(|v: &mut Vec<_>| v.push((i, j)))
                                .or_insert(vec![(i, j)]);
                            Cell::Antenna(c)
                        }
                    )
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

        Ok(Map {
            cells,
            antennas,
        })
    }
}

fn main() {
    let map: Map = INPUT.try_into().unwrap();

    let antinodes1 = map.find_unique_antinodes(Map::antinodes1);

    println!("{:?}", antinodes1.len());

    let antinodes2 = map.find_unique_antinodes(Map::antinodes_infinity);

    println!("{:?}", antinodes2.len());
}