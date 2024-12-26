use std::collections::HashSet;

const INPUT: &str = include_str!("./day25.txt");

#[derive(Debug)]
enum Schematic<const W: usize, const H: usize> {
    Lock([u32; W]),
    Key([u32; W]),
}

impl<const W: usize, const H: usize> Schematic<W, H> {
    fn fit(x: [u32; W], y: [u32; W]) -> bool {
        (0..W).all(|i| x[i] + y[i] <= (H as u32) - 2)
    }
}

impl<const W: usize, const H: usize> TryFrom<&str> for Schematic<W, H> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tiles: Vec<Vec<char>> = value.lines()
            .map(|line| {
                line.chars().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut heights = [0u32; W];

        if tiles.len() != H {
            return Err("meh");
        }

        // XXX: Assume there's no malformed locks nor keys, e.g., ignore
        // non-contiguous `#` characters in a column.
        for x in 0..W {
            heights[x] = (0..H)
                .filter(|&y| {
                    tiles[y][x] == '#'
                })
                .count() as u32 - 1;
        }

        if tiles[0].iter().all(|&c| c == '#') {
            Ok(Schematic::Lock(heights))
        } else if tiles[0].iter().all(|&c| c == '.') {
            Ok(Schematic::Key(heights))
        } else {
            Err("ding")
        }
    }
}


fn main() {
    let schematics = INPUT.split("\n\n")
        .filter_map(|schematic| {
            Schematic::<5, 7>::try_from(schematic).ok()
        })
        .collect::<Vec<_>>();

    let (locks, keys) = schematics.into_iter()
        .fold((
            HashSet::<[u32; 5]>::new(),
            HashSet::<[u32; 5]>::new()
        ), |(mut locks, mut keys), s| {
            match s {
                Schematic::Lock(s) => locks.insert(s),
                Schematic::Key(s) => keys.insert(s),
            };

            (locks, keys)
        });

    let res = keys.iter().map(|&key| {
        locks.iter().filter(|&&lock| Schematic::<5, 7>::fit(key, lock)).count()
    })
    .sum::<usize>();

    println!("{}", res);
}