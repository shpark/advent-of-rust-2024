const PROGRAM: &str = include_str!("./day3.txt");

const MUL: &str = "mul(";

const DO: &str = "do()";

const DONT: &str = "don't()";

fn part1(program: &str) -> usize {
    let mut i = 0usize;
    let mut res = 0usize;

    while i < program.len() {
        let suffix = &program[i..];
        let mut operand1 = 0usize;
        let mut operand2 = 0usize;

        if !suffix.starts_with(MUL) {
            i += 1;
            continue;
        }

        i += MUL.len();
        let suffix = &program[i..];

        // XXX: ugly
        if let Some(j) = suffix.find(',') {
            if let Ok(op1) = &suffix[..j].parse::<usize>() {
                operand1 = *op1;
                i += j + 1;
            } else {
                continue;
            }
        } else {
            continue;
        }

        let suffix = &program[i..];

        if let Some(j) = suffix.find(')') {
            if let Ok(op2) = &suffix[..j].parse::<usize>() {
                operand2 = *op2;
                i += j + 1;
            } else {
                continue;
            }
        } else {
            continue;
        }

        res += operand1 * operand2;
    }

    res
}

fn part2(program: &str) -> usize {
    let mut i = 0usize;
    let mut res = 0usize;

    let mut is_enabled = true;
    while i < program.len() {
        let suffix = &program[i..];
        let mut operand1 = 0usize;
        let mut operand2 = 0usize;

        if suffix.starts_with(DO) {
            is_enabled = true;
            i += DO.len();
            continue;
        } else if suffix.starts_with(DONT) {
            is_enabled = false;
            i += DONT.len();
            continue;
        }

        if !suffix.starts_with(MUL) {
            i += 1;
            continue;
        }

        i += MUL.len();
        let suffix = &program[i..];

        // XXX: ugly
        if let Some(j) = suffix.find(',') {
            if let Ok(op1) = &suffix[..j].parse::<usize>() {
                operand1 = *op1;
                i += j + 1;
            } else {
                continue;
            }
        } else {
            continue;
        }

        let suffix = &program[i..];

        if let Some(j) = suffix.find(')') {
            if let Ok(op2) = &suffix[..j].parse::<usize>() {
                operand2 = *op2;
                i += j + 1;
            } else {
                continue;
            }
        } else {
            continue;
        }

        if is_enabled {
            res += operand1 * operand2;
        }
    }

    res
}

fn main() {
    let sum: usize = PROGRAM.lines().map(part1).sum();

    println!("{}", sum);

    let sum: usize = part2(&PROGRAM);

    println!("{}", sum);
}