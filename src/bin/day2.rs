use itertools::Itertools;

const INPUT: &str = include_str!("./day2.txt");

fn is_path_safe(levels: &[i32]) -> bool {
    let mut is_safe = true;

    is_safe &= levels.windows(2).all(|pair| {
        let diff = pair[0].abs_diff(pair[1]);
        (1 <= diff) && (diff <= 3)
    });

    // WARNING: inefficient
    is_safe &= levels.windows(2).all(|pair| pair[0] < pair[1]) ||
        levels.windows(2).all(|pair| pair[0] > pair[1]);

    is_safe
}

fn main() {
    // part 1
    let num_safe_paths = INPUT.split('\n').filter(|line| {
        let levels: Vec<i32> = line.split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        is_path_safe(&levels)
    })
    .count();

    println!("{}", num_safe_paths);

    // part 2
    // WARNING: inefficient
    let num_safe_paths = INPUT.split('\n').filter(|line| {
        let levels: Vec<i32> = line.split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let k = levels.len() - 1;
        let levels_with_damper: Vec<Vec<_>> = levels.into_iter()
            .combinations(k)
            .collect();

        levels_with_damper.iter().any(|levels| is_path_safe(&levels))
    })
    .count();

    println!("{}", num_safe_paths);
}