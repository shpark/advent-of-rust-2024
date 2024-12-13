const INPUT: &str = include_str!("./day13.txt");

const OFFSET: i64 = 10000000000000;

#[derive(Debug)]
struct Buttons<const A: u64, const B: u64> {
    a: (i64, i64),
    b: (i64, i64),
}

// a.0 * x + b.0 * y = prize.0
// a.1 * x + b.1 * y = prize.1
//
// ( a.0 b.0 ) (x) = (prize.0)
// ( a.1 b.1 ) (y)   (prize.1)
//
// (x) = ( a.0 b.0 )^-1 (prize.0)
// (y)   ( a.1 b.1 )    (prize.1)
//
// (x) =  (a.0 * b.1 - b.0 * a.1)^(-1) * (  b.1 -b.0 ) (prize.0)
// (y)                                   ( -a.1  a.0 ) (prize.1)

impl<const A: u64, const B: u64> Buttons<A, B> {
    fn solve(
        &self,
        prize: (i64, i64)
    ) -> Option<u64> {
        let d = self.a.0 * self.b.1 - self.b.0 * self.a.1;

        let d_times_x = self.b.1 * prize.0 - self.b.0 * prize.1;
        let d_times_y = -self.a.1 * prize.0 + self.a.0 * prize.1;

        if d_times_x % d == 0 && d_times_y % d == 0 {
            let x = d_times_x / d;
            let y = d_times_y / d;

            if x >= 0 && y >= 0 {
                return Some(A * (x as u64) + B * (y as u64));
            }
        }

        None
    }
}

#[derive(Debug)]
struct Eqn<const A: u64, const B: u64, const D: i64> {
    buttons: Buttons<A, B>,
    prize: (i64, i64),
}

impl<const A: u64, const B: u64, const D: i64> Eqn<A, B, D> {
    fn solve(&self) -> Option<u64> {
        self.buttons.solve(self.prize)
    }
}

impl<const A: u64, const B: u64, const D: i64> TryFrom<&str> for Eqn<A, B, D> {
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

                prize.0 += D;
                prize.1 += D;
            }
        });

        Ok(Eqn {
            buttons: Buttons { a, b },
            prize,
        })
    }
}

fn main() {
    let part1 = INPUT.split("\n\n")
        .filter_map(|s| {
            let eqn: Eqn<3, 1, 0> = s.try_into().unwrap();

            eqn.solve()
        })
        .sum::<u64>();

    println!("{}", part1);

    let part2 = INPUT.split("\n\n")
        .filter_map(|s| {
            let eqn: Eqn<3, 1, OFFSET> = s.try_into().unwrap();

            eqn.solve()
        })
        .sum::<u64>();

    println!("{}", part2);
}