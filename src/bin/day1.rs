use std::{collections::HashMap, iter::zip};

const INPUT: &str = include_str!("./day1.txt");

fn main() {
    let mut left = vec![];
    let mut right = vec![];

    for line in INPUT.split("\n") {
        let tokens: Vec<_> = line.split("   ").collect();

        left.push(tokens[0].parse::<i32>().unwrap());
        right.push(tokens[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let res = zip(left.iter(), right.iter()).fold(0, |acc, (l, r)| {
        acc + l.abs_diff(*r)
    });

    println!("{}", res);

    let mut hist: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|key| {
        hist.entry(*key)
        .and_modify(|v| *v += 1)
        .or_insert(1i32);
    });

    let similarity = left.iter().fold(0, |acc, v| {
        acc + v * hist.get(v).unwrap_or(&0)
    });
    
    println!("{}", similarity);
}