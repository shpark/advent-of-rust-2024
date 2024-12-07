const PUZZLE: &str = include_str!("./day4.txt");

const DIRS: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[inline]
fn peek_next(
    puzzle: &Vec<Vec<char>>,
    curr: (usize, usize),
    dir: &(i32, i32)
) -> Option<(char, (usize, usize))> {
    // XXX: I am lazy and won't give shit to signed integer overflow, etc.
    let row: i64 = (curr.0 as i64) + (dir.0 as i64);
    let col: i64 = (curr.1 as i64) + (dir.1 as i64);

    // Check index validity
    if row < 0 || row >= puzzle.len() as i64 ||
        col < 0 || col >= puzzle[0].len() as i64 {
        return None;
    }

    // Then peek
    Some((puzzle[row as usize][col as usize], (row as usize, col as usize)))
}

fn search(
    puzzle: &Vec<Vec<char>>,
    src: (usize, usize),
    dir: &(i32, i32),
) -> bool {
    if puzzle[src.0][src.1] != 'X' {
        return false;
    }

    if let Some(('M', p)) = peek_next(puzzle, src, dir) {
        if let Some(('A', p)) = peek_next(puzzle, p, dir) {
            if let Some(('S', _)) = peek_next(puzzle, p, dir) {
                return true;
            }
        }
    } 

    false
}

fn search2(
    puzzle: &Vec<Vec<char>>,
    src: (usize, usize),
) -> bool {
    if puzzle[src.0][src.1] != 'A' {
        return false;
    }

    // Cases
    // M.. S..
    // .A. .A.
    // ..S ..M ...squared

    if (puzzle[src.0 - 1][src.1 - 1] == 'M' &&
        puzzle[src.0 + 1][src.1 + 1] == 'S') ||
        (puzzle[src.0 + 1][src.1 + 1] == 'M' &&
        puzzle[src.0 - 1][src.1 - 1] == 'S') {
        if (puzzle[src.0 - 1][src.1 + 1] == 'M' &&
            puzzle[src.0 + 1][src.1 - 1] == 'S') ||
            (puzzle[src.0 + 1][src.1 - 1] == 'M' &&
            puzzle[src.0 - 1][src.1 + 1] == 'S') {
            return true;
    }
}

    return false;
}

fn main() {
    let puzzle: Vec<Vec<char>> = PUZZLE
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let indices: Vec<_> = (0..puzzle.len())
        .flat_map(|i| (0..puzzle[0].len()).map(move |j| (i, j)))
        .collect();

    assert_eq!(indices.len(), puzzle.len() * puzzle[0].len());

    let num_found: usize = indices.into_iter()
        .map(|(i, j)| DIRS.iter()
            .filter(|dir| search(&puzzle, (i, j), dir)).count())
        .sum();

    println!("{}", num_found);

    let indices: Vec<_> = (1..(puzzle.len() - 1))
        .flat_map(|i| (1..(puzzle[0].len() - 1)).map(move |j| (i, j)))
        .collect();

    let num_found: usize = indices.into_iter()
        .filter(|&(i, j)| search2(&puzzle, (i, j)))
        .count();

    println!("{}", num_found);
}