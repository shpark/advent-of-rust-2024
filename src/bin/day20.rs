use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./day20.txt");

const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
}

struct RaceTrack {
    start: (usize, usize),
    end: (usize, usize),
    tiles: Vec<Vec<Tile>>,
    elapsed_ps_no_cheat: usize,
    dist_map: HashMap<(usize, usize), usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Cheat {
    cheat_start: (usize, usize),
    cheat_end: (usize, usize),
}

impl RaceTrack {
    fn bfs(&self, src: (usize, usize)) -> HashMap<(usize, usize), usize> {
        let mut dist = HashMap::<(usize, usize), usize>::new();
        dist.insert(src.clone(), 0);

        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_front(src.clone());

        while !q.is_empty() {
            let (y, x) = q.pop_back().unwrap();

            let d = *dist.get(&(y, x)).unwrap();

            for &(dy, dx) in DIRS {
                let (y1, x1) = (
                    (y as i32 + dy) as usize,
                    (x as i32 + dx) as usize,
                );

                match self.tiles[y1][x1] {
                    Tile::Track => {
                        if !dist.contains_key(&(y1, x1)) {
                            dist.insert((y1, x1), d + 1);
                            q.push_front((y1, x1));
                        }
                    },
                    Tile::Wall => {},
                }
            }
        }

        dist
    }

    fn populate_dist_map(&mut self)  {
        self.dist_map = self.bfs(self.end);
        self.elapsed_ps_no_cheat = *self.dist_map.get(&self.start).unwrap();
    }

    fn pos_at_dist(
        &self,
        (y, x): (usize, usize),
        dist: usize
    ) -> Vec<(usize, usize)> {
        let mut res = Vec::<(usize, usize)>::new();

        for dy in 0..=dist as i32 {
            let dx = dist as i32 - dy;

            for &(sy, sx) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let (y1, x1) = (y as i32 + dy * sy, x as i32 + dx * sx);

                if y1 < 0 || y1 >= self.tiles.len() as i32 ||
                    x1 < 0 || x1 >= self.tiles[0].len() as i32 {
                    continue;
                }

                let (y1, x1) = (y1 as usize, x1 as usize);

                if self.tiles[y1][x1] == Tile::Track {
                    res.push((y1, x1));
                }
            }
        }

        res
    }

    fn cheats_to_saved_ps(&self, budget: usize) -> HashMap<Cheat, usize> {
        let mut cheats_to_dist = HashMap::<Cheat, usize>::new();

        let mut dist = HashMap::<(usize, usize), usize>::new();
        dist.insert(self.start, 0);

        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_front(self.start);

        while !q.is_empty() {
            let cheat_start = q.pop_back().unwrap();

            let before_cheat = *dist.get(&cheat_start).unwrap();

            for len_cheat in 0..=budget {
                for cheat_end in self.pos_at_dist(cheat_start, len_cheat) {
                    let after_cheat = self.dist_map.get(&cheat_end).unwrap();

                    let elapsed_ps_with_cheat = before_cheat +
                        len_cheat +
                        after_cheat;

                    let saved_ps = self.elapsed_ps_no_cheat as i32 -
                        elapsed_ps_with_cheat as i32;

                    if saved_ps > 0 {
                        cheats_to_dist.insert(
                            Cheat { cheat_start, cheat_end },
                            saved_ps as usize,
                        );
                    }
                }
            }

            for &(dy, dx) in DIRS {
                let (y1, x1) = (
                    (cheat_start.0 as i32 + dy) as usize,
                    (cheat_start.1 as i32 + dx) as usize,
                );

                match self.tiles[y1][x1] {
                    Tile::Track => {
                        if !dist.contains_key(&(y1, x1)) {
                            dist.insert((y1, x1), before_cheat + 1);
                            q.push_front((y1, x1));
                        }
                    },
                    Tile::Wall => {},
                }
            }
        }

        cheats_to_dist
    }
}

impl TryFrom<&str> for RaceTrack {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let tiles = input.lines()
            .map(|line| line.chars()
                    .map(|c| if c == '#' {
                        Tile::Wall
                    } else {
                        Tile::Track
                    })
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();

        let mut start = (0usize, 0usize);
        let mut end = (0usize, 0usize);
        let mut cheats = Vec::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if y <= 0 || y >= tiles.len() - 1 ||
                    x <= 0 || x >= tiles.len() - 1 {
                    return;
                }

                if c == 'S' {
                    start = (y, x);
                } else if c == 'E' {
                    end = (y, x);
                } else if c == '#' {
                    // collect possible cheats
                    let maybe_cheat = DIRS.iter().any(|(dy, dx)| {
                        let (y1, x1) = (y as i32 + dy, x as i32 + dx);

                        if y1 <= 0 || y1 > (tiles.len() - 1) as i32 ||
                            x1 <= 0 || x1 > (tiles[0].len() - 1) as i32 {
                            return false;
                        }

                        let (y1, x1) = (y1 as usize, x1 as usize);

                        tiles[y1][x1] != Tile::Wall
                    });

                    if maybe_cheat {
                        cheats.push((y, x));
                    }
                }
            });
        });

        Ok(RaceTrack {
            start,
            end,
            tiles,
            elapsed_ps_no_cheat: 0,
            dist_map: HashMap::new()
        })
    }
}

fn main() {
    let mut race_track = RaceTrack::try_from(INPUT).unwrap();

    race_track.populate_dist_map();

    let cheat_to_saved_ps = race_track.cheats_to_saved_ps(2);

    let res = cheat_to_saved_ps.into_iter()
        .filter(|&(_cheat, saved_ps)| saved_ps >= 100)
        .count();

    println!("{}", res);

    let cheat_to_saved_ps = race_track.cheats_to_saved_ps(20);

    let res = cheat_to_saved_ps.into_iter()
        .filter(|&(_cheat, saved_ps)| saved_ps >= 100)
        .count();

    println!("{}", res);
}
