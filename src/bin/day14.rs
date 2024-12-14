use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("./day14.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Robot<const W: usize, const H: usize> {
    p: (i64, i64),
    v: (i64, i64),
}

impl<const W: usize, const H: usize> Robot<W, H> {
    #[allow(dead_code)]
    fn update(&mut self) {
        self.p = (
            (self.p.0 + self.v.0).rem_euclid(W as i64),
            (self.p.1 + self.v.1).rem_euclid(H as i64)
        );
    }

    fn update_n(&mut self, n: i64) {
        self.p = (
            (self.p.0 + n * self.v.0).rem_euclid(W as i64),
            (self.p.1 + n * self.v.1).rem_euclid(H as i64)
        );
    }

    // For convenience, assign each quadrant with id:
    // +----->
    // | 1 2
    // | 3 4
    // v
    // NOTE: This function assumes W and H are both odd numbers.
    fn quadrant(&self) -> Option<u8> {
        let mw = (W as i64) / 2;
        let mh = (H as i64) / 2;

        match (self.p.0.cmp(&mw), self.p.1.cmp(&mh)) {
            (Ordering::Less, Ordering::Less) => Some(1),
            (Ordering::Greater, Ordering::Less) => Some(2),
            (Ordering::Less, Ordering::Greater) => Some(3),
            (Ordering::Greater, Ordering::Greater) => Some(4),
            _ => None
        }
    }
}

impl<const W: usize, const H: usize> TryFrom<&str> for Robot<W, H> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_whitespace().collect::<Vec<_>>();

        let p = {
            let s = parts[0].split(',').collect::<Vec<_>>();
            (
                s[0][2..s[0].len()].parse::<i64>().unwrap(),
                s[1].parse::<i64>().unwrap(),
            )
        };

        let v = {
            let s = parts[1].split(',').collect::<Vec<_>>();
            (
                s[0][2..s[0].len()].parse::<i64>().unwrap(),
                s[1].parse::<i64>().unwrap(),
            )
        };

        Ok(Robot { p, v })
    }
}

fn main() {
    let mut num_robots_in_quadrant: HashMap<u8, usize> = HashMap::new();

    INPUT.lines()
        .for_each(|line| {
            let mut robot: Robot<101, 103> = line.try_into().unwrap();

            robot.update_n(100);

            if let Some(quadrant) = robot.quadrant() {
                num_robots_in_quadrant.entry(quadrant)
                    .and_modify(|n| *n += 1)
                    .or_insert(1usize);
            }
        });

    println!("{:?}", num_robots_in_quadrant.values().product::<usize>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_n() {
        let mut robot: Robot<11, 7> = Robot { p: (2, 4), v: (2, -3) };

        let mut robot2 = robot.clone();

        for _ in 0..7 {
            robot.update();
        }

        robot2.update_n(7);
    }
}