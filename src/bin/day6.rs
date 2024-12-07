use std::collections::HashSet;

const INPUT: &str = include_str!("./day6.txt");

#[derive(Debug)]
struct Guard<'a> {
    at: (usize, usize),
    dir: Dir,
    // stupid?
    visited: HashSet<((usize, usize), Dir)>,
    map: &'a Vec<Vec<Cell>>,
}

enum StepResult {
    Skip,
    OutOfMap,
    Loop,
}

impl<'a> Guard<'a> {
    fn new(at: (usize, usize), map: &'a Vec<Vec<Cell>>) -> Self {
        Guard {
            at,
            dir: Dir::Up,
            visited: HashSet::new(),
            map,
        }
    }

    fn step(&mut self) -> StepResult {
        if self.visited.contains(&(self.at, self.dir)) {
            return StepResult::Loop;
        }

        self.visited.insert((self.at, self.dir));

        let next = (
            (self.at.0 as i32) + (self.dir.delta().0),
            (self.at.1 as i32) + (self.dir.delta().1),
        );

        if next.0 < 0 || next.0 >= self.map.len() as i32 ||
            next.1 < 0 || next.1 >= self.map[0].len() as i32 {
            return StepResult::OutOfMap;
        }

        let next = (next.0 as usize, next.1 as usize);

        match self.map[next.0][next.1].kind {
            CellKind::Empty => {
                self.at = next;
            },
            CellKind::Occupied=> {
                self.turn_right();
            },
        }

        StepResult::Skip
    }

    #[inline]
    fn turn_right(&mut self) {
        self.dir = self.dir.next_dir();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
        }
    }

    fn next_dir(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug)]
enum CellKind {
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Cell {
    kind: CellKind,
}

fn main() {
    // parse map
    let mut guard_pos = (0usize, 0usize);

    let mut map = INPUT.lines().enumerate()
        .map(|(i, row)| row.chars().enumerate().map(|(j, cell)| {
            if cell == '^' {
                guard_pos = (i, j);
            }

            Cell {
                kind: if cell == '#' {
                    CellKind::Occupied
                } else {
                    CellKind::Empty
                }
            }
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Search initial guard position
    let mut guard = Guard::new(guard_pos, &map);

    while !matches!(guard.step(), StepResult::OutOfMap) {
    }

    let num_unique_pos = guard.visited.iter()
        .map(|(p, _)| *p)
        .collect::<HashSet<_>>();

    println!("{}", num_unique_pos.len());

    let blockers = num_unique_pos.into_iter().filter(|&(i, j)| {
        map[i][j].kind = CellKind::Occupied;

        let mut guard = Guard::new(guard_pos, &map);

        let is_blocker = loop {
            match guard.step() {
                StepResult::Loop => break true,
                StepResult::OutOfMap => break false,
                StepResult::Skip => {},
            }
        };

        map[i][j].kind = CellKind::Empty;

        is_blocker
    }).collect::<Vec<_>>();

    println!("{:?}", blockers.len());
}