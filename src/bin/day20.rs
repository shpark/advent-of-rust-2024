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
    cheats: Vec<(usize, usize)>,
}

impl RaceTrack {
    fn bfs(&self) -> (
        HashMap<(usize, usize), usize>,
        HashMap<(usize, usize), (usize, usize)>
    ) {
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        dist.insert(self.start, 0);
        q.push_front(self.start);

        while !q.is_empty() {
            let (y, x) = q.pop_back().unwrap();

            'next_dir: for (dy, dx) in DIRS.iter() {
                let (y1, x1) = (
                    (y as i32 + dy) as usize,
                    (x as i32 + dx) as usize
                );

                if self.tiles[y1][x1] == Tile::Wall {
                    continue 'next_dir;
                }

                if dist.get(&(y1, x1)).is_none() {
                    prev.insert((y1, x1), (y, x));
                    dist.insert((y1, x1), dist.get(&(y, x)).unwrap() + 1);
                    q.push_front((y1, x1));
                }
            }
        }

        (dist, prev)
    }

    fn check_cheat(
        &mut self,
        cheat: (usize, usize),
        orig_record: usize,
    ) -> Option<usize> {
        self.tiles[cheat.0][cheat.1] = Tile::Track;

        let (dist, _prev) = self.bfs();

        self.tiles[cheat.0][cheat.1] = Tile::Wall;

        if let Some(&new_record) = dist.get(&self.end) {
            if new_record < orig_record {
                return Some(orig_record - new_record);
            }
        }

        None
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

        Ok(RaceTrack { start, end, tiles, cheats })
    }
}

fn main() {
    let mut race_track: RaceTrack = INPUT.try_into().unwrap();

    let orig_record = *race_track.bfs().0.get(&race_track.end).unwrap();

    let mut num_cheats_by_saved_ps: HashMap<usize, usize> = HashMap::new();

    for &cheat in race_track.cheats.clone().iter() {
        if let Some(saved) = race_track.check_cheat(cheat, orig_record) {
            num_cheats_by_saved_ps.entry(saved)
                .and_modify(|s| { *s += 1; })
                .or_insert(1);
        }
    }

    let res = num_cheats_by_saved_ps.iter()
        .filter_map(|(&k, v)| {
            if k >= 100 { Some(v) } else { None }
        })
        .sum::<usize>();

    println!("{}", res);
}
