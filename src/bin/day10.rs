use std::collections::HashSet;

const INPUT: &str = include_str!("./day10.txt");

const DIRS: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

#[derive(Debug)]
struct Position {
    height: u32,
    rating: Option<u32>,
    reachable: HashSet<(usize, usize)>,
}

impl Position {
    fn is_trailhead(&self) -> bool {
        self.height == 0
    }
}

#[derive(Debug)]
struct TopographicMap {
    positions: Vec<Vec<Position>>,
}

impl TopographicMap {
    #[inline]
    fn height(&self, x: usize, y: usize) -> u32 {
        self.positions[x][y].height
    }

    #[inline]
    fn rating(&self, x: usize, y: usize) -> Option<u32> {
        self.positions[x][y].rating
    }

    #[inline]
    fn next_positions(&self, curr: (usize, usize)) -> Vec<(usize, usize)> {
        DIRS.iter()
            .filter_map(|&(dx, dy)| {
                let x = (curr.0 as i32) + dx;
                let y = (curr.1 as i32) + dy;

                if x < 0 || x >= (self.positions.len() as i32) ||
                    y < 0 || y >= (self.positions[0].len() as i32) {
                    return None;
                }

                let x = x as usize;
                let y = y as usize;

                if self.height(x, y) == self.height(curr.0, curr.1) + 1 {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn update(&mut self, curr: (usize, usize)) {
        if self.positions[curr.0][curr.1].height == 9 {
            self.positions[curr.0][curr.1].reachable.insert(curr);
            self.positions[curr.0][curr.1].rating.replace(1);
            return;
        }

        if let Some(_) = self.positions[curr.0][curr.1].rating {
            return;
        }

        self.positions[curr.0][curr.1].rating = Some(0);

        self.next_positions(curr).iter().for_each(|&n| {
            self.update(n);

            // part1: score (i.e., # reachable peaks)
            // FIXME(efficiency): clone
            let reachable_from_n = &self.positions[n.0][n.1].reachable.clone();
            self.positions[curr.0][curr.1].reachable.extend(reachable_from_n);

            // part 2: rating
            let new_rating = match self.positions[curr.0][curr.1].rating {
                Some(rating) => rating + self.rating(n.0, n.1).unwrap(),
                None => self.rating(n.0, n.1).unwrap()
            };
            self.positions[curr.0][curr.1].rating.replace(new_rating);
        });
    }
}

impl TryFrom<&str> for TopographicMap {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let positions = input.lines()
            .map(|row| {
                row.chars()
                    .map(|c| Position {
                        height: c.to_digit(10).unwrap(),
                        rating: None,
                        reachable: HashSet::new(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(TopographicMap { positions })
    }
}

fn main() {
    let mut topomap: TopographicMap = INPUT.try_into().unwrap();

    for i in 0..topomap.positions.len() {
        for j in 0..topomap.positions[0].len() {
            topomap.update((i, j));
        }
    }

    let sum_trailhead_scores = topomap.positions.iter()
        .map(|row| {
            row.iter()
                .filter(|p| p.is_trailhead())
                .map(|p| p.reachable.len() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", sum_trailhead_scores);

    let sum_trailhead_ratings = topomap.positions.iter()
        .map(|row| {
            row.iter()
                .filter(|p| p.is_trailhead())
                .map(|p| p.rating.unwrap_or(0))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", sum_trailhead_ratings);
}