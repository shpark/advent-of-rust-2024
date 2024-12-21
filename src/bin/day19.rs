use std::collections::HashSet;

const INPUT: &str = include_str!("./day19.txt");

#[derive(Debug)]
struct Patterns<'a>(Vec<&'a str>);

struct Parser<'a, 'b> {
    patterns: Patterns<'a>,
    cache: HashSet<&'b str>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn try_parse(&mut self, input: &'b str) -> Result<(), &'static str> {
        if self.patterns.0.contains(&input) {
            self.cache.insert(input);
            return Ok(());
        }

        for &pattern in self.patterns.0.clone().iter() {
            if input.starts_with(pattern) {
                if let Ok(_) = self.try_parse(&input[pattern.len()..]) {
                    self.cache.insert(input);
                    return Ok(())
                }
            }
        }

        Err("meh")
    }
}

fn main() {
    let parts = INPUT.split("\n\n").collect::<Vec<_>>();

    let mut parser = Parser {
        patterns: Patterns(parts[0].split(", ").collect::<Vec<_>>()),
        cache: HashSet::new(),
    };

    let res = parts[1].lines()
        .filter(|input|  parser.try_parse(input).is_ok())
        .count();

    println!("{}", res);
}