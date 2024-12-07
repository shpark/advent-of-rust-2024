use std::{cmp::Ordering, collections::HashMap, fmt};

const INPUT: &str = include_str!("./day5.txt");

struct OrderingRules(HashMap<i32, Vec<i32>>);

struct Page<'a> {
    ctx: &'a OrderingRules,
    value: i32,
}

impl<'a> fmt::Display for Page<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Page {{ {} }}", self.value)
    }
}

impl<'a> PartialEq for Page<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ctx, other.ctx) && self.value == other.value
    }
}

impl<'a> PartialOrd for Page<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !std::ptr::eq(self.ctx, other.ctx) {
            return None;
        }

        if let Some(larger_than_self) = self.ctx.0.get(&self.value) {
            if larger_than_self.contains(&other.value) {
                return Some(Ordering::Less)
            }
        } else {
            return Some(Ordering::Greater);
        }

        if let Some(larger_than_other) = other.ctx.0.get(&other.value) {
            if larger_than_other.contains(&self.value) {
                return Some(Ordering::Greater)
            }
        } else {
            return Some(Ordering::Less);
        }

        None
    }
}

fn main() {
    let (rules, updates) =  {
        let parts = INPUT.split("\n\n").collect::<Vec<_>>();
        (parts[0], parts[1])
    };

    // Parse ordering rules
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules.lines() {
        let (from, to) = {
            let parts = rule.split('|')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (parts[0], parts[1])
        };

        ordering_rules.entry(from)
            .and_modify(|xs| xs.push(to))
            .or_insert(vec![to]);
    }

    let ctx = OrderingRules(ordering_rules);

    let mut part1 = 0i32;
    let mut part2 = 0i32;

    updates.lines()
        .map(|update| {
            update.split(',')
                .map(|s| {
                    Page { ctx: &ctx, value: s.parse::<i32>().unwrap() }
                })
                .collect::<Vec<_>>()
        })
        .for_each(|mut pages| {
            let is_sorted = (0..(pages.len() - 1))
                .all(|i| {
                    let p = &pages[i];
                    let q = &pages[i + 1];
                    p.partial_cmp(q) == Some(Ordering::Less)
                });
            
            if is_sorted {
                part1 += pages[pages.len() / 2].value;
            } else {
                pages.sort_by(|p, q| p.partial_cmp(q).unwrap());
                part2 += pages[pages.len() / 2].value
            }
        });

    println!("{}", part1);

    println!("{}", part2);
}
