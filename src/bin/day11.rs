const INPUT: &str = include_str!("./day11.txt");

const MAGIC: u64 = 2024;

#[derive(Debug)]
struct Stone(u64);

impl Stone {
    fn bilnk(self) -> Vec<Stone> {
        if self.0 == 0 {
            vec![Stone(1)]
        } else if self.0.to_string().len() % 2 == 0 {
            let s = self.0.to_string();

            vec![
                Stone(s[0..s.len() / 2].parse::<u64>().unwrap()),
                Stone(s[(s.len() / 2)..s.len()].parse::<u64>().unwrap()),
            ]
        } else {
            vec![Stone(self.0 * MAGIC)]
        }
    }
}

fn main() {
    let mut stones = INPUT.split_whitespace()
        .map(|n| Stone(n.parse().unwrap()))
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones.into_iter().flat_map(|s| s.bilnk()).collect::<Vec<_>>();
    }

    println!("{}", stones.len());
}
