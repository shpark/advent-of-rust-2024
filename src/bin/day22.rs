use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./day22.txt");

#[derive(PartialEq, Eq, Debug)]
struct Changes {
    inner: VecDeque<i8>,
}

impl Changes {
    fn new() -> Self {
        Self { inner: VecDeque::new() }
    }

    fn push(&mut self, value: i8) {
        self.inner.push_back(value as i8);

        if self.inner.len() > 4 {
            let _ = self.inner.pop_front();
        }
    }

    fn packed(&self) -> Option<u32> {
        if self.inner.len() < 4 {
            None
        } else {
            Some(
                (((self.inner[0] as u32) & 0xff) << 24) as u32 |
                (((self.inner[1] as u32) & 0xff) << 16) as u32 |
                (((self.inner[2] as u32) & 0xff) << 8) as u32 |
                ((self.inner[3]) as u32 & 0xff)
            )
        }
    }
}

impl From<u32> for Changes {
    fn from(mut value: u32) -> Self {
        let mut inner = VecDeque::new();

        for _ in 0..4 {
            inner.push_front((value & 0xff) as i8);
            value = value >> 8;
        }

        Self { inner }
    }
}

struct Prng {
    secret: u64,
    changes: Changes,
}

impl Prng {
    fn from(seed: u64) -> Self {
        Self { secret: seed, changes: Changes::new() }
    }

    // To mix a value into the secret number, calculate the bitwise XOR of
    // given value and the secret number. Then, the secret number becomes the
    // result of that operation. (If the secret number is 42 and you were to mix
    // 15 into the secret number, the secret number would become 37.)
    fn mix(&mut self, value: u64) {
        self.secret ^= value;
    }

    // To prune the secret number, calculate the value of the secret number
    // modulo 16777216. Then, the secret number becomes the result of that
    // operation. (If the secret number is 100000000 and you were to prune the
    // secret number, the secret number would become 16113920.)
    fn prune(&mut self) {
        self.secret = self.secret.rem_euclid(16777216);
    }

    fn evolve(&mut self) {
        let prev = self.secret;

        // Calculate the result of multiplying the secret number by 64. Then,
        // mix this result into the secret number. Finally, prune the secret
        // number.
        self.mix(self.secret << 6);
        self.prune();

        // Calculate the result of dividing the secret number by 32. Round the
        // result down to the nearest integer. Then, mix this result into the
        // secret number. Finally, prune the secret number.
        self.mix(self.secret.div_euclid(32));
        self.prune();

        // Calculate the result of multiplying the secret number by 2048. Then,
        // mix this result into the secret number. Finally, prune the secret
        // number.
        self.mix(self.secret * 2048);
        self.prune();

        self.changes.push(
            (((self.secret % 10) as i32) - ((prev % 10) as i32)) as i8,
        );
    }
}

fn main() {
    // part 1
    let res = INPUT.lines().map(|n| {
        let mut prng = Prng::from(n.parse::<u64>().unwrap());
        for _ in 0..2000 {
            prng.evolve();
        }
        prng.secret
    })
    .sum::<u64>();

    println!("{}", res);

    // part 2
    let mut changes_to_bananas = HashMap::<u32, HashMap<u64, u64>>::new();

    INPUT.lines().for_each(|seed| {
        let seed = seed.parse::<u64>().unwrap();

        let mut prng = Prng::from(seed);

        for _ in 0..2000 {
            prng.evolve();

            if let Some(packed) = prng.changes.packed() {
                changes_to_bananas.entry(packed)
                    .and_modify(|seed_to_bananas| {
                        if !seed_to_bananas.contains_key(&seed) {
                            seed_to_bananas.insert(seed, prng.secret % 10);
                        }
                    })
                    .or_insert({
                        let mut seed_to_bananas = HashMap::new();
                        seed_to_bananas.insert(seed, prng.secret % 10);
                        seed_to_bananas
                    });
            }
        }
    });

    let (changes, seed_to_bananas) = changes_to_bananas.into_iter()
        .max_by(|(_p1, m1), (_p2, m2)| {
            m1.values().sum::<u64>().cmp(&m2.values().sum::<u64>())
        })
        .unwrap();

    println!("{}", seed_to_bananas.values().sum::<u64>());
}
