use std::collections::HashMap;

const INPUT: &str = include_str!("./day19.txt");

#[derive(Debug)]
struct Patterns<'a>(Vec<&'a str>);

struct Parser<'a, 'b> {
    patterns: Patterns<'a>,
    cache: HashMap<&'b str, usize>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn warm_up(&mut self) {
        self.cache.insert("", 1);
    }

    fn try_parse(&mut self, input: &'b str) -> Result<usize, &'static str> {
        if let Some(&k) = self.cache.get(input) {
            return Ok(k);
        }

        let mut num_representations = 0usize;

        for &pattern in self.patterns.0.clone().iter() {
            if input.starts_with(pattern) {
                if let Ok(k) = self.try_parse(&input[pattern.len()..]) {
                    num_representations += k;
                }
            }
        }

        if num_representations > 0 {
            self.cache.insert(input, num_representations);
            Ok(num_representations)
        } else {
            Err("meh")
        }
    }
}

fn main() {
    let parts = INPUT.split("\n\n").collect::<Vec<_>>();

    let mut parser = Parser {
        patterns: Patterns(parts[0].split(", ").collect::<Vec<_>>()),
        cache: HashMap::new(),
    };

    parser.warm_up();

    let inputs = parts[1].lines().collect::<Vec<_>>();

    let res = inputs.iter()
        .filter(|input| parser.try_parse(input).is_ok())
        .count();

    println!("{}", res);

    let res = parser.cache.iter().filter_map(|(k, &v)| {
        if inputs.contains(k) { Some(v) } else { None }
    })
    .sum::<usize>();

    println!("{}", res);
}