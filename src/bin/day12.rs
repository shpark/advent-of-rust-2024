use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./day12.txt");

#[derive(PartialEq, Eq, Debug)]
struct Plant(char);

#[derive(Debug)]
struct Garden {
    plots: Vec<Vec<Plant>>
}

const DIRS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Garden {
    fn plots_in_a_same_region(
        &self,
        src: (usize, usize),
        visited: &mut HashSet<(usize, usize)>
    ) -> Vec<(i32, i32)> {
        let mut res = vec![(src.0 as i32, src.1 as i32)];

        visited.insert(src);

        for &(dx, dy) in DIRS {
            let x = src.0 as i32 + dx;
            let y = src.1 as i32 + dy;

            if x < 0 || x >= self.plots.len() as i32 ||
                y < 0 || y >= self.plots[0].len() as i32 {
                continue;
            }

            let (x, y) = (x as usize, y as usize);

            if self.plots[x][y] == self.plots[src.0][src.1] &&
                !visited.contains(&(x, y)) {
                res.extend(self.plots_in_a_same_region(
                    (x as usize, y as usize),
                    visited)
                );
            }
        }

        res
    }

    fn regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..self.plots.len() {
            for j in 0..self.plots[0].len() {
                if visited.contains(&(i, j)) {
                    continue;
                }

                regions.push(Region::new(
                    self.plots_in_a_same_region((i, j), &mut visited)
                ));
            }
        }

        regions
    }
}

#[derive(Debug)]
struct Region {
    area: u64,
    perimeter: u64,
    sides: u64,
}

const DIAGS: &[(i32, i32)] = &[
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Region {
    fn new(plots: Vec<(i32, i32)>) -> Self {
        let mut fences: HashMap<(i32, i32), u64> = HashMap::new();

        for &(x, y) in &plots {
            for &(dx, dy) in DIRS {
                let fx = (x as i32) + dx;
                let fy = (y as i32) + dy;

                if !plots.contains(&(fx, fy)) {
                    fences.entry((fx, fy))
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
        }

        let mut corners = 0u64;

        for &(x, y) in &plots {
            for &(dx, dy) in DIAGS {
                let p = plots.contains(&(x as i32 + dx, y as i32));
                let q = plots.contains(&(x as i32, y as i32 + dy));
                let r = plots.contains(&(x as i32 + dx, y as i32 + dy));

                // The rationale behind addign the third term here is kinda
                // hacky. It would help addressing cases like:
                //
                //     AAAAAA
                //     AAABBA
                //     AAABBA
                //     ABBAAA
                //     ABBAAA
                //     AAAAAA
                //
                // TODO?: Add explanation
                if (!r && p && q) || (!r && !p && !q) || (r && !p && !q) {
                    corners += 1;
                }
            }
        }

        Region {
            area: plots.len() as u64,
            perimeter: fences.values().sum(),
            sides: corners,
        }
    }
}

fn main() {
    let garden = Garden {
        plots: INPUT.lines()
            .map(|row| row.chars().map(Plant).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };

    let regions = garden.regions();

    println!("{}", regions.iter()
        .map(|reg| reg.area * reg.perimeter)
        .sum::<u64>()
    );

    println!("{}", regions.iter()
        .map(|reg| reg.area * reg.sides)
        .sum::<u64>()
    );
}
