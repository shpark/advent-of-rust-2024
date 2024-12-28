use std::collections::HashSet;

const INPUT: &str = include_str!("./day24-corrected.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Wire<'a> {
    X(usize),
    Y(usize),
    Z(usize),
    Other(&'a str),
}

impl<'a> TryFrom<&'a str> for Wire<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err("meh");
        }

        if value.starts_with("x") {
            Ok(Wire::X(value[1..3].parse::<usize>().unwrap()))
        } else if value.starts_with("y") {
            Ok(Wire::Y(value[1..3].parse::<usize>().unwrap()))
        } else if value.starts_with("z") {
            Ok(Wire::Z(value[1..3].parse::<usize>().unwrap()))
        } else {
            Ok(Wire::Other(value))
        }
    }
}

impl<'a> std::fmt::Display for Wire<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wire::X(n) => write!(f, "x[{}]", n),
            Wire::Y(n) => write!(f, "y[{}]", n),
            Wire::Z(n) => write!(f, "z[{}]", n),
            Wire::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for GateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateKind::And => write!(f, "&"),
            GateKind::Or => write!(f, "|"),
            GateKind::Xor => write!(f, "^"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Gate<'a> {
    kind: GateKind,
    a: Wire<'a>,
    b: Wire<'a>,
    o: Wire<'a>,
}

impl<'a> TryFrom<&'a str> for Gate<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts = value.split_whitespace().collect::<Vec<_>>();

        let a: Wire<'a> = parts[0].try_into().unwrap();
        let b: Wire<'a> = parts[2].try_into().unwrap();
        let o: Wire<'a> = parts[4].try_into().unwrap();

        let kind = match parts[1] {
            "AND" => GateKind::And,
            "OR" => GateKind::Or,
            "XOR" => GateKind::Xor,
            _ => return Err("ding"),
        };

        Ok(Gate { kind, a, b, o })
    }
}

impl<'a> std::fmt::Display for Gate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "assign {} = {} {} {};", self.o, self.a, self.kind, self.b)
    }
}

struct FruitMonitor<'a, const I: usize, const O: usize> {
    wires: HashSet<Wire<'a>>,
    gates: HashSet<Gate<'a>>,
}

impl<'a, const I: usize, const O: usize> TryFrom<&'a str> for FruitMonitor<'a, I, O> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut wires = HashSet::<Wire>::new();
        let mut gates = HashSet::<Gate>::new();

        value.split("\n\n").skip(1).next().unwrap().lines()
            .for_each(|gate| {
                let gate: Gate<'a> = gate.try_into().unwrap();

                wires.insert(gate.a.clone());
                wires.insert(gate.b.clone());
                wires.insert(gate.o.clone());

                gates.insert(gate);
            });

        Ok(Self {
            wires,
            gates,
        })
    }
}

impl <'a, const I: usize, const O: usize> FruitMonitor<'a, I, O> {
    fn find_consumers(&self, wire: &Wire<'a>) -> Vec<&Gate<'a>> {
        let mut consumers = Vec::<&Gate>::new();

        for gate in self.gates.iter() {
            if wire == &gate.a || wire == &gate.b {
                consumers.push(gate);
            }
        }

        consumers
    }

    // By looking at the number of gates and intermediate wires, we could
    // conclude that the circuit is the textbook 45-bit ripple-carry
    // full adder using only AND, OR and XOR gates.
    //
    // This function collects suspicious gates that does not satisfy some
    // properties that a gate within a well-formed ripple-carry full-adder
    // should.
    fn find_sus(&self) -> HashSet<&Gate<'a>> {
        let mut sus = HashSet::<&Gate<'a>>::new();

        for gate in self.gates.iter() {
            match gate.kind {
                GateKind::And => {
                    match (gate.a, gate.b) {
                        (Wire::X(0), Wire::Y(0)) |
                        (Wire::Y(0), Wire::X(0)) => {},

                        (Wire::X(i), Wire::Y(j)) |
                        (Wire::Y(j), Wire::X(i)) if i == j => {
                            let gs = self.find_consumers(&gate.o);
                            if gs.len() != 1 ||
                                (gs.len() > 0 && gs[0].kind != GateKind::Or) {
                                sus.insert(gate);
                            }
                        },
                        (Wire::Other(_), Wire::Other(_)) => {
                            let gs = self.find_consumers(&gate.o);
                            if gs.len() != 1 ||
                                (gs.len() > 0 && gs[0].kind != GateKind::Or) {
                                sus.insert(gate);
                            }
                        },
                        _ => {},
                    }
                },
                GateKind::Or => {
                    match (gate.a, gate.b) {
                        (Wire::Other(_), Wire::Other(_)) => {
                            if &gate.o == &Wire::Z(O - 1) {
                                continue;
                            }

                            let gs = self.find_consumers(&gate.o);

                            if gs.len() != 2 {
                                sus.insert(gate);
                            }

                            let has_xor_consumer = gs.iter().any(|g| {
                                g.kind == GateKind::Xor &&
                                matches!(g.o, Wire::Z(_))
                            });

                            let has_and_consumer = gs.iter().any(|g| {
                                g.kind == GateKind::And
                            });

                            if !(has_xor_consumer && has_and_consumer) {
                                sus.insert(gate);
                            }
                        },
                        _ => {
                            sus.insert(gate);
                        }
                    }
                },
                GateKind::Xor => {
                    match (gate.a, gate.b) {
                        (Wire::X(0), Wire::Y(0)) |
                        (Wire::Y(0), Wire::X(0)) => {},

                        (Wire::X(i), Wire::Y(j)) |
                        (Wire::Y(j), Wire::X(i)) if i == j => {
                            let gs = self.find_consumers(&gate.o);
                            if gs.len() != 2 {
                                sus.insert(gate);
                            }

                            let has_xor_consumer = gs.iter().any(|g| {
                                g.kind == GateKind::Xor &&
                                matches!(g.o, Wire::Z(_))
                            });

                            let has_and_consumer = gs.iter().any(|g| {
                                g.kind == GateKind::And
                            });

                            if !(has_xor_consumer && has_and_consumer) {
                                sus.insert(gate);
                            }
                        },
                        _ => {},
                    }
                },
            }
        }

        sus
    }
}

impl <'a, const I: usize, const O: usize> std::fmt::Display for FruitMonitor<'a, I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\
module FruitMonitor (
    input wire [{}:0] x,
    input wire [{}:0] y,
    output wire [{}:0] z
);
",
            I - 1, I - 1, O - 1
        )?;

        for wire in self.wires.iter() {
            match wire {
                Wire::Other(w) => writeln!(f, "    wire {};", w),
                _ => Ok(()),
            }?;
        };

        writeln!(f, "")?;

        for gate in self.gates.iter() {
            writeln!(f, "    {}", gate)?;
        }

        writeln!(f, "\nendmodule")
    }
}

fn main() {
    let fruit_monitor: FruitMonitor<45, 46> = FruitMonitor::try_from(INPUT)
        .unwrap();

    // XXX: This would print the verilog module for the fruit monitor...
    // println!("{}", fruit_monitor);

    let sus = fruit_monitor.find_sus();

    for gate in sus.iter() {
        println!("{:?}", gate);
    }

    // XXX: Wire pairs to swap are manually found based on the output :)
    // The corrected circuit can be found in day24-corrected.txt.
}
