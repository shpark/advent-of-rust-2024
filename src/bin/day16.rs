use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./day16-sample.txt");

// The Reindeer start on the Start Tile (marked S) facing East and need to
// reach the End Tile (marked E). They can move forward one tile at a time
// (increasing their score by 1 point), but never into a wall (#). They can
// also rotate clockwise or counterclockwise 90 degrees at a time (increasing
// their score by 1000 points).

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    End,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Reindeer {
    p: (usize, usize),
    dir: Dir,
}

impl Reindeer {
    fn neighbors(&self) -> Vec<(Reindeer, usize)> {
        let mut neighbors: Vec<(Reindeer, usize)> = Vec::new();

        neighbors.push({
                let delta = self.dir.delta();
                let p = (
                    ((self.p.0 as i32) + delta.0) as usize,
                    ((self.p.1 as i32) + delta.1) as usize,
                );
                (Reindeer { p, dir: self.dir }, 1)
        });

        neighbors.push((
            Reindeer {
                p: self.p,
                dir: self.dir.rotate_right(),
            },
            1000,
        ));

        neighbors.push((
            Reindeer {
                p: self.p,
                dir: self.dir.rotate_left(),
            },
            1000,
        ));

        neighbors
    }
}

struct Maze {
    reindeer: Reindeer,
    exit: (usize, usize),
    tiles: Vec<Vec<Tile>>,
    dist_map: HashMap<(Reindeer, Reindeer), usize>,
}

impl Maze {
    fn lowest_score(&self) -> Option<usize> {
        let mut dist: HashMap<Reindeer, usize> = HashMap::new();
        let mut q: VecDeque<Reindeer> = VecDeque::new();

        dist.insert(self.reindeer.clone(), 0);
        q.push_front(self.reindeer.clone());

        while !q.is_empty() {
            let reindeer = q.pop_back().unwrap();

            for (n, d) in reindeer.neighbors() {
                if self.tiles[n.p.0][n.p.1] == Tile::Wall {
                    continue;
                }

                let dn = dist.get(&reindeer).unwrap() + d;

                if dn == *dist.entry(n)
                    .and_modify(|d| *d = if *d > dn { dn } else { *d })
                    .or_insert(dn) {
                    q.push_front(n);
                }
            }
        }

        dist.into_iter()
            .filter_map(|(reindeer, d)| if reindeer.p == self.exit {
                Some(d)
            } else {
                None
            })
            .min()
    }

    fn populate_dist_map(
        &mut self,
        src: Reindeer,
        dst: Reindeer,
    ) {
        let mut q: VecDeque<Reindeer> = VecDeque::new();

        println!("{:?}", src);

        self.dist_map.insert((src.clone(), src.clone()), 0);
        q.push_front(self.reindeer.clone());

        while !q.is_empty() {
            let reindeer = q.pop_back().unwrap();

            for (n, d) in reindeer.neighbors() {
                if self.tiles[n.p.0][n.p.1] == Tile::Wall {
                    continue;
                }

                if let Some(&n_to_dst) = self.dist_map.get(&(n, dst)) {
                    let dn = n_to_dst - d;

                    self.dist_map.entry((src.clone(), dst.clone()))
                        .and_modify(|d| *d = if *d > dn { dn } else { *d })
                        .or_insert(dn);

                    continue;
                }

                let dn = self.dist_map.get(&(src, reindeer)).unwrap() + d;

                if dn == *self.dist_map.entry((src, reindeer))
                    .and_modify(|d| *d = if *d > dn { dn } else { *d })
                    .or_insert(dn) {
                    q.push_front(n);
                }
            }
        }
    }

    fn count_tiles_on_shortest_path(&mut self) -> usize {
        let dirs = vec![Dir::North, Dir::South, Dir::East, Dir::West];

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                for dir in dirs.iter() {
                    self.populate_dist_map(
                        self.reindeer,
                        Reindeer { p: (i, j), dir: dir.clone() });
                }
            }
        }

        let lowest_score = self.lowest_score().unwrap();

        let mut res = 0usize;

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                let mut is_on_shortest_path = false;

                'outer: for dir1 in dirs.iter() {
                    for dir2 in dirs.iter() {
                        let r = Reindeer { p: (i, j), dir: dir1.clone() };

                        let d1 = self.dist_map.get(&(
                            self.reindeer,
                            r
                        ));

                        let d2 = self.dist_map.get(&(
                            r,
                            Reindeer { p: self.exit, dir: dir2.clone() }
                        ));

                        match (d1, d2) {
                            (Some(&d1), Some(&d2)) => if d1 + d2 == lowest_score {
                                is_on_shortest_path = true;
                                break 'outer;
                            },
                            _ => {},
                        }
                    }
                }

                if is_on_shortest_path {
                    println!("{:?} is on one of the shortest paths", (i, j));
                    res += 1;
                }
            }
        }

        res
    }
}

impl TryFrom<&str> for Maze {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut reindeer = Reindeer {
            p: (0usize, 0usize),
            dir: Dir::East,
        };

        let mut exit = (0usize, 0usize);

        let tiles = input.lines().enumerate()
            .map(|(y, row)| {
                row.chars().enumerate().map(|(x, c)| 
                    match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty, 
                        'S' => {
                            reindeer.p = (y, x);
                            Tile::Empty
                        }
                        'E' => {
                            exit = (y, x);
                            Tile::End
                        },
                        _ => panic!("ding"),
                    }
                ).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Maze {
            reindeer,
            exit,
            tiles,
            dist_map: HashMap::new(),
        })
    }
}

fn main() {
    let mut maze: Maze = INPUT.try_into().unwrap();

    let res = maze.lowest_score();

    println!("{}", res.unwrap());

    let res = maze.count_tiles_on_shortest_path();

    println!("{}", res);

    println!("{:?}", maze.dist_map);
}