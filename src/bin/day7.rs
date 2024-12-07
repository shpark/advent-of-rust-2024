const INPUT: &str = include_str!("./day7.txt");

struct Eqn {
    target: i64,
    operands: Vec<i64>,
}

enum BinOp {
    Add,
    Mul,
    Concat,
}

impl BinOp {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            BinOp::Add => lhs + rhs,
            BinOp::Mul => lhs * rhs,
            BinOp::Concat => {
                // FIXME(efficiency :p)
                format!("{}{}", lhs, rhs).parse::<i64>().unwrap()
            },
        }
    }
}

impl Eqn {
    fn is_sat(&self, binops: &Vec<BinOp>) -> bool {
        self.operands.iter()
            .fold(Vec::new(), |acc, &operand| {
                if acc.len() == 0 {
                    return vec![operand];
                }

                acc.into_iter()
                    .flat_map(|value| {
                        binops.iter()
                            .map(|binop| binop.eval(value, operand))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .contains(&self.target)
    }
}

impl TryFrom<&str> for Eqn {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(':').collect::<Vec<_>>();

        let target = parts[0].parse::<i64>().unwrap();

        let operands = parts[1].split_whitespace()
            .map(|operand| operand.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Eqn { target, operands })
    }
}

fn main() {
    let part1 = INPUT.lines()
        .map(|line| Eqn::try_from(line).unwrap())
        .filter(|eqn| eqn.is_sat(&vec![BinOp::Add, BinOp::Mul]))
        .map(|eqn| eqn.target)
        .sum::<i64>();

    println!("{}", part1);

    let part2 = INPUT.lines()
        .map(|line| Eqn::try_from(line).unwrap())
        .filter(|eqn| eqn.is_sat(&vec![BinOp::Add, BinOp::Mul, BinOp::Concat]))
        .map(|eqn| eqn.target)
        .sum::<i64>();

    println!("{}", part2);
}