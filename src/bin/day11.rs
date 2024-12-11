use std::collections::HashMap;

const INPUT: &str = include_str!("./day11.txt");

const MAGIC: u64 = 2024;

const K1: usize = 25;

const K2: usize = 75;

#[derive(Debug)]
struct Stone(u64);

impl Stone {
    // F(n, k) = (# stones after blinking stone N k times)
    //
    // - forall n. F(n, 0) = 1
    // - F(0, 1) = 1
    // - F(1, 1) = 1
    // - forall n. len(n.to_string()) % 2 == 0 -> F(n, 1) = 2
    // - forall n. len(n.to_string()) % 2 == 1 -> F(n, 1) = 1
    //
    // - forall n. if let (upper, lower) = n -> F(n, k) = F(uppser, k) + F(lower, k - 1)
    //             else if n = 0 -> F(n, k) = F(1, k - 1)
    //             else F(n, k) = F(2024 * n, k - 1)
    fn blink_k(&self, k: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
        if let Some(&n) = cache.get(&(self.0, k)) {
            return n;
        }

        if k == 0 {
            return 1;
        }

        let mut res = 0usize;

        if self.0.to_string().len() % 2 == 0 {
            let s = self.0.to_string();

            res += Stone(s[0..(s.len() / 2)].parse().unwrap()).blink_k(k - 1, cache);
            res += Stone(s[(s.len() / 2)..s.len()].parse().unwrap()).blink_k(k - 1, cache);
        } else if self.0 == 0 {
            res += Stone(1).blink_k(k - 1, cache);
        } else {
            res += Stone(self.0 * MAGIC).blink_k(k - 1, cache);
        }

        cache.insert((self.0, k), res);

        res
    }
}

fn main() {
    let stones = INPUT.split_whitespace()
        .map(|n| Stone(n.parse().unwrap()))
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let res = stones.iter()
        .map(|s| s.blink_k(K1, &mut cache))
        .sum::<usize>();

    println!("{}", res);

    let res = stones.iter()
        .map(|s| s.blink_k(K2, &mut cache))
        .sum::<usize>();

    println!("{}", res);
}
