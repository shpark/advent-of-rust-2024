const INPUT: &str = include_str!("./day13.txt");

const NUM_MAX_PRESSES: usize = 100;

#[derive(Debug)]
struct Buttons<const A: usize, const B: usize> {
    a: (i64, i64),
    b: (i64, i64),
}

impl<const A: usize, const B: usize> Buttons<A, B> {
    fn solve(
        &self,
        prize: (i64, i64)
    ) -> Option<usize> {
        let mut sat: Vec<(usize, usize)> = Vec::new();

        for na in 0..NUM_MAX_PRESSES as i64 {
            for nb in 0..NUM_MAX_PRESSES as i64 {
                if prize.0 == na * self.a.0 + nb * self.b.0 &&
                    prize.1 == na * self.a.1 + nb * self.b.1 {
                    sat.push((na as usize, nb as usize));
                }
            }
        }

        sat.iter().map(|(a, b)| A * a + B * b).min()
    }
}

#[derive(Debug)]
struct Eqn<const A: usize, const B: usize> {
    buttons: Buttons<A, B>,
    prize: (i64, i64),
}

impl<const A: usize, const B: usize> Eqn<A, B> {
    fn solve(&self) -> Option<usize> {
        self.buttons.solve(self.prize)
    }
}

impl<const A: usize, const B: usize> TryFrom<&str> for Eqn<A, B> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut a = (0i64, 0i64);
        let mut b = (0i64, 0i64);
        let mut prize = (0i64, 0i64);

        value.lines().enumerate().for_each(|(i, line)| {
            if i < 2 {
                let parts = line.split_whitespace().collect::<Vec<_>>();

                let p = (
                    parts[2][2..(parts[2].len() - 1)].parse().unwrap(),
                    parts[3][2..parts[3].len()].parse().unwrap(),
                );

                if i == 0 { a = p; } else { b = p; }
            } else {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                prize.0 = parts[1][2..(parts[1].len() - 1)].parse().unwrap();
                prize.1 = parts[2][2..parts[2].len()].parse().unwrap();
            }
        });

        Ok(Eqn {
            buttons: Buttons { a, b },
            prize,
        })
    }
}

fn main() {
    let num_tokens_to_get_all_possible_prizes = INPUT.split("\n\n")
        .filter_map(|s| {
            let eqn: Eqn<3, 1> = s.try_into().unwrap();

            eqn.solve()
        })
        .sum::<usize>();

    println!("{}", num_tokens_to_get_all_possible_prizes);
}