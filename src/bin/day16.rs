use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, usize};

use itertools::Itertools;

const INPUT: &str = include_str!("./day16.txt");

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

    fn dirs() -> Vec<Dir> {
        vec![Dir::East, Dir::South, Dir::West, Dir::North]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Node {
    p: (usize, usize),
    dir: Dir,
}

impl Node {
    fn rotate_right(&self) -> Self {
        Node { p: self.p, dir: self.dir.rotate_right() }
    }

    fn rotate_left(&self) -> Self {
        Node { p: self.p, dir: self.dir.rotate_left() }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Edge {
    dst: Node,
    cost: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

struct Maze {
    src: Node,
    exit: (usize, usize),
    adj_list: HashMap<Node, Vec<Edge>>,
}

impl Maze {
    fn dijkstra(&self) -> (HashMap<Node, usize>, HashMap<Node, HashSet<Node>>) {
        let mut pq: BinaryHeap<Edge> = BinaryHeap::new();
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();

        dist.insert(self.src, 0);
        prev.insert(self.src, HashSet::new());
        pq.push(Edge { dst: self.src, cost: 0 });

        while let Some(Edge { dst: u, cost: _cost }) = pq.pop() {
            if let Some(neighbors) = self.adj_list.get(&u) {
                for &Edge { dst: v, cost } in neighbors {
                    let alt = dist.get(&u).unwrap_or(&usize::MAX) + cost;

                    let res = dist.entry(v)
                        .and_modify(|dist_v| if alt < *dist_v {
                            *dist_v = alt;
                        })
                        .or_insert(alt);

                    if *res == alt {
                        pq.push(Edge { dst: v, cost: alt });
                        prev.entry(v)
                            .and_modify(|p: &mut HashSet<Node>| { p.insert(u); })
                            .or_insert({ 
                                let mut s = HashSet::new();
                                s.insert(u);
                                s
                            });
                    }
                }
            }
        }

        (dist, prev)
    }
}

impl TryFrom<&str> for Maze {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut adj_list: HashMap<Node, Vec<Edge>> = HashMap::new();

        let mut src = (0usize, 0usize);
        let mut exit = (0usize, 0usize);

        let tiles = input.lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in 0..tiles.len() {
            'next_tile: for x in 0..tiles[0].len() {
                if tiles[y][x] == '#' {
                    continue 'next_tile;
                }

                if tiles[y][x] == 'E' {
                    exit = (y, x);
                }

                if tiles[y][x] == 'S' {
                    src = (y, x);
                }

                for dir in Dir::dirs().into_iter() {
                    let node = Node { p: (y, x), dir };
                    let mut successors = vec![
                        Edge { dst: node.rotate_right(), cost: 1000 },
                        Edge { dst: node.rotate_left(), cost: 1000 },
                    ];

                    let (dy, dx) = dir.delta();
                    let (y1, x1) = (
                        (y as i32 + dy) as usize,
                        (x as i32 + dx) as usize,
                    );

                    if tiles[y1][x1] != '#' {
                        successors.push(Edge {
                            dst: Node { p: (y1, x1), dir },
                            cost: 1,
                        });
                    }
                    
                    adj_list.insert(node, successors);
                }
            }
        }

        Ok(Maze {
            src: Node { p: src, dir: Dir::East },
            exit,
            adj_list,
       })
    }
}

fn find_pos_on_shortest_path(
    maze: &Maze,
    len_shortest_path: usize,
    dist: &HashMap<Node, usize>,
    prev: &HashMap<Node, HashSet<Node>>,
) -> HashSet<(usize, usize)> {
    let mut nodes_on_shortest_path: HashSet<Node> = HashSet::new();

    for dir in Dir::dirs() {
        let maybe_exit_node = Node { p: maze.exit, dir };

        if *dist.get(&maybe_exit_node).unwrap() == len_shortest_path {
            println!("{:?} can be an exit node", maybe_exit_node);

            nodes_on_shortest_path.insert(maybe_exit_node);

            let mut q = VecDeque::new();
            q.push_front(maybe_exit_node);

            while !q.is_empty() {
                if let Some(n) = q.pop_back() {
                    for &p in prev.get(&n).unwrap() {
                        if !nodes_on_shortest_path.contains(&p) {
                            nodes_on_shortest_path.insert(p);
                            q.push_front(p);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", nodes_on_shortest_path.len());

    nodes_on_shortest_path.into_iter()
        .map(|n| n.p)
        .collect::<HashSet<_>>()
}

fn highlight_shortest_path_on_maze(
    original: &str,
    pos_on_the_shortest_path: &HashSet<(usize, usize)>,
) -> String {
    original.lines().enumerate()
        .map(|(y, row)| {
            row.chars().enumerate()
                .map(|(x, c)| {
                    if pos_on_the_shortest_path.contains(&(y, x)) {
                        'O'
                    } else {
                        c
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

fn main() {
    let maze: Maze = INPUT.try_into().unwrap();

    let (dist, prev) = maze.dijkstra();

    let len_shortest_path = dist.iter()
        .filter_map(|(&n, &d)| {
            if n.p == maze.exit {
                Some(d)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("{}", len_shortest_path);

    let pos_on_shortest_path = find_pos_on_shortest_path(
        &maze, len_shortest_path, &dist, &prev
    );

    println!("{}", pos_on_shortest_path.len());

    println!("{}", highlight_shortest_path_on_maze(INPUT, &pos_on_shortest_path));
}

#[cfg(test)]
mod test {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_parse() {
        let maze: Maze = "\
#####
#..E#
#...#
#S..#
#####".try_into().unwrap();

        for (k, v) in maze.adj_list {
            println!("{:?} -> {:?}", k, v);
        }
    }

    #[test]
    fn test_edge_min_heap() {
        let mut pq: BinaryHeap<Edge> = BinaryHeap::new();

        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 3 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 5 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 1 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 2 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 4 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 7 });
        pq.push(Edge { dst: Node { p: (0, 0), dir: Dir::East }, cost: 6 });

        while !pq.is_empty() {
            let x = pq.pop().unwrap();
            println!("{:?}", x);
        }
    }

    #[test]
    fn test_shortest_path() {
        let maze: Maze = "\
#####
#..E#
#...#
#S..#
#####".try_into().unwrap();

        let (dist, prev) = maze.dijkstra();

        for (n, cost) in dist {
            println!("{:?} -> {}", n, cost);
        }

        for (n, predecessors) in prev {
            println!("{:?} -> {:?}", n, predecessors);
        }
    }
}
