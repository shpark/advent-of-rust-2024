use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./day18.txt");

const DIRS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Byte {
    Safe,
    Corrupted,
}

struct Ram<const N: usize>([[Byte; N]; N]);

impl<const N: usize> Ram<N> {
    fn new() -> Self {
        Self([[Byte::Safe; N]; N])
    }

    fn capacity(&self) -> usize {
        N
    }

    fn corrupt(&mut self, y: usize, x: usize) {
        self.0[y][x] = Byte::Corrupted;
    }

    fn bfs(&self) -> HashMap<(usize, usize), usize>{
        let mut dist = HashMap::new();
        let mut q = VecDeque::new();

        dist.insert((0, 0), 0);
        q.push_front((0, 0));

        while !q.is_empty() {
            let u = q.pop_back().unwrap();
            let d = *dist.get(&u).unwrap();

            'next_dir: for &(dy, dx) in DIRS.iter() {
                let (y, x) = (u.0 as i32 + dy, u.1 as i32 + dx);
                if y < 0 || N <= y as usize || x < 0 || N <= x as usize {
                    continue 'next_dir;
                }

                let (y, x) = (y as usize, x as usize);
                if self.0[y][x] == Byte::Corrupted {
                    continue 'next_dir;
                }

                if let None = dist.get(&(y, x)) {
                    dist.insert((y, x), d + 1);
                    q.push_front((y, x));
                }
            }
        }

        dist
    }
}

impl<const N: usize> std::fmt::Display for Ram<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            for j in 0..N {
                write!(f, "{}", match self.0[i][j] {
                    Byte::Safe => ".",
                    Byte::Corrupted => "#",
                })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut ram: Ram<71> = Ram::new();

    let n = ram.capacity();

    // part 1
    INPUT.lines().take(1024).for_each(|line| {
        let parts = line.split(",").collect::<Vec<_>>();
        let (y, x) = (
            parts[1].parse::<usize>().unwrap(),
            parts[0].parse::<usize>().unwrap(),
        );
        ram.corrupt(y, x);
    });

    let dist = ram.bfs();

    println!("{}", dist.get(&(n - 1, n - 1)).unwrap());

    // part 2
    let bytes_falling = INPUT.lines().map(|line| {
        let parts = line.split(",").collect::<Vec<_>>();
        let (y, x) = (
            parts[1].parse::<usize>().unwrap(),
            parts[0].parse::<usize>().unwrap(),
        );
        (y, x)
    }).collect::<Vec<_>>();

    let indices = (0..bytes_falling.len()).collect::<Vec<_>>();

    let p = indices.partition_point(|&k| {
        let mut ram: Ram<71> = Ram::new();

        for i in 0..k {
            let (y, x) = bytes_falling[i];
            ram.corrupt(y, x);
        }

        ram.bfs().get(&(n - 1, n - 1)).is_some()
    });

    println!("{},{}", bytes_falling[p - 1].1, bytes_falling[p - 1].0);
}
