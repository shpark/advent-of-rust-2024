const INPUT: &str = include_str!("./day21.txt");

const NUMERIC_KEYPAD: NumericKeypad = NumericKeypad;

const DIRECTIONAL_KEYPAD: DirectionalKeypad = DirectionalKeypad;

use std::collections::{HashMap, VecDeque};

struct DirectionalKeypad;

struct NumericKeypad;

trait Keypad {
    type Button: KeypadButton;

    fn get_pos(button: &Self::Button) -> (i32, i32);

    fn get_button(&self, pos: &(i32, i32)) -> Option<Self::Button>;

    fn helper(
        end: (i32, i32),
        rpath: &Vec<(i32, i32)>,
        lineage: &HashMap<(i32, i32), Vec<(i32, i32)>>,
    ) -> Vec<Vec<(i32, i32)>> {
        let curr = rpath.iter()
            .fold(end, |acc, delta| (acc.0 - delta.0, acc.1 - delta.1));

        if let Some(deltas) = lineage.get(&curr) {
            deltas.into_iter()
                .map(|&delta| {
                    let mut rpath = rpath.clone();
                    rpath.push(delta);
                    rpath
                })
                .collect::<Vec<_>>()
        } else {
            vec![rpath.clone()]
        }
    }

    fn lineage_to_paths(
        d: usize,
        end: (i32, i32),
        lineage: &HashMap<(i32, i32), Vec<(i32, i32)>>
    ) -> Vec<Vec<(i32, i32)>> {
        let mut rpaths: Vec::<Vec<(i32, i32)>> = vec![vec![]];

        for _ in 0..d {
            rpaths = rpaths.iter()
                .flat_map(|rpath| Self::helper(end, rpath, lineage))
                .collect::<Vec<Vec<_>>>();
        }

        rpaths.iter_mut().for_each(|rpath| rpath.reverse());

        rpaths.into_iter().collect::<Vec<_>>()
    }


    fn find_shortest_paths_between(
        &self,
        src: &Self::Button,
        dst: &Self::Button,
    ) -> Vec<Vec<(i32, i32)>> {
        let mut dist = HashMap::<(i32, i32), usize>::new();
        dist.insert(Self::get_pos(src), 0);

        let mut q = VecDeque::<(i32, i32)>::new();
        q.push_front(Self::get_pos(src));

        let mut lineage = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();

        // Run BFS once to populates `deltas` map.
        while !q.is_empty() {
            let (y, x) = q.pop_back().unwrap();

            let dv = dist.get(&(y, x)).unwrap().clone();

            for &(dy, dx) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (y1, x1) = (y + dy, x + dx);

                if let Some(_button) = self.get_button(&(y1, x1)) {
                    if let Some(&du0) = dist.get(&(y1, x1)).clone() {
                        if dv + 1 <= du0 {
                            lineage.entry((y1, x1)).and_modify(|s| {
                                s.push((dy, dx));
                            });
                        }
                    } else {
                        dist.insert((y1, x1), dv + 1);

                        lineage.insert(
                            (y1, x1),
                            {
                                let mut s = Vec::new();
                                s.push((dy, dx));
                                s
                            }
                        );
                        q.push_front((y1, x1));
                    }
                }
            }
        }

        let shortest_paths = Self::lineage_to_paths(
            *dist.get(&Self::get_pos(dst)).unwrap(),
            Self::get_pos(dst),
            &lineage,
        );

        shortest_paths
    }
}

#[allow(dead_code)]
trait KeypadButton: Default + std::fmt::Display {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DirectionalKeypadButton {
    Up,
    Activate,
    Left,
    Down,
    Right,
}

impl std::fmt::Display for DirectionalKeypadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Down => "v",
            Self::Up => "^",
            Self::Left  => "<",
            Self::Right => ">",
            Self::Activate => "A",
        })
    }
}

impl Default for DirectionalKeypadButton {
    fn default() -> Self {
        Self::Activate
    }
}

impl KeypadButton for DirectionalKeypadButton {}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
impl Keypad for DirectionalKeypad {
    type Button = DirectionalKeypadButton;

    fn get_pos(button: &DirectionalKeypadButton) -> (i32, i32) {
        match button {
            DirectionalKeypadButton::Activate => (0, 2),
            DirectionalKeypadButton::Up => (0, 1),
            DirectionalKeypadButton::Left => (1, 0),
            DirectionalKeypadButton::Down => (1, 1),
            DirectionalKeypadButton::Right => (1, 2),
        }
    }

    fn get_button(&self, &(y, x): &(i32, i32)) -> Option<Self::Button> { match (y, x) {
            (0, 0) => None,
            (0, 1) => Some(DirectionalKeypadButton::Up),
            (0, 2) => Some(DirectionalKeypadButton::Activate),
            (1, 0) => Some(DirectionalKeypadButton::Left),
            (1, 1) => Some(DirectionalKeypadButton::Down),
            (1, 2) => Some(DirectionalKeypadButton::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum NumericKeypadButton {
    Num(u8),
    Activate,
}

impl TryFrom<char> for NumericKeypadButton {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0'..='9' => Ok(
                NumericKeypadButton::Num(c as u8 - '0' as u8),
            ),
            'A' => Ok(NumericKeypadButton::Activate),
            _ => Err("ding"),
        }
    }
}

impl std::fmt::Display for NumericKeypadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericKeypadButton::Activate => write!(f, "A"),
            NumericKeypadButton::Num(n) => write!(f, "{}", n),
        }
    }
}

impl Default for NumericKeypadButton {
    fn default() -> Self {
        Self::Activate
    }
}

impl KeypadButton for NumericKeypadButton {}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
impl Keypad for NumericKeypad {
    type Button = NumericKeypadButton;

    fn get_pos(button: &Self::Button) -> (i32, i32) {
        match button {
            NumericKeypadButton::Num(0) => (3, 1),
            NumericKeypadButton::Num(1) => (2, 0),
            NumericKeypadButton::Num(2) => (2, 1),
            NumericKeypadButton::Num(3) => (2, 2),
            NumericKeypadButton::Num(4) => (1, 0),
            NumericKeypadButton::Num(5) => (1, 1),
            NumericKeypadButton::Num(6) => (1, 2),
            NumericKeypadButton::Num(7) => (0, 0),
            NumericKeypadButton::Num(8) => (0, 1),
            NumericKeypadButton::Num(9) => (0, 2),
            NumericKeypadButton::Activate => (3, 2),
            _ => panic!(),
        }
    }

    fn get_button(&self, &(y, x): &(i32, i32)) -> Option<Self::Button> {
        match (y, x) {
            (0, 0) => Some(NumericKeypadButton::Num(7)),
            (0, 1) => Some(NumericKeypadButton::Num(8)),
            (0, 2) => Some(NumericKeypadButton::Num(9)),
            (1, 0) => Some(NumericKeypadButton::Num(4)),
            (1, 1) => Some(NumericKeypadButton::Num(5)),
            (1, 2) => Some(NumericKeypadButton::Num(6)),
            (2, 0) => Some(NumericKeypadButton::Num(1)),
            (2, 1) => Some(NumericKeypadButton::Num(2)),
            (2, 2) => Some(NumericKeypadButton::Num(3)),
            (3, 0) => None,
            (3, 1) => Some(NumericKeypadButton::Num(0)),
            (3, 2) => Some(NumericKeypadButton::Activate),
            _ => None,
        }
    }
}

trait Controller {
    type Command: Clone;

    fn step_to_cmd(step: &(i32, i32)) -> Self::Command;
}

enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&(i32, i32)> for Dirs {
    type Error = &'static str;

    fn try_from(&(dy, dx): &(i32, i32)) -> Result<Self, Self::Error> {
        match (dy, dx) {
            (1, 0) => Ok(Self::Down),
            (-1, 0) => Ok(Self::Up),
            (0, 1) => Ok(Self::Right),
            (0, -1) => Ok(Self::Left),
            _ => Err("ding"),
        }
    }
}

impl Controller for DirectionalKeypad {
    type Command = DirectionalKeypadButton;

    fn step_to_cmd(step: &(i32, i32)) -> Self::Command {
        match Dirs::try_from(step).unwrap() {
            Dirs::Up => DirectionalKeypadButton::Up,
            Dirs::Down  => DirectionalKeypadButton::Down,
            Dirs::Left  => DirectionalKeypadButton::Left,
            Dirs::Right => DirectionalKeypadButton::Right,
        }
    }
}

trait KeypadWithController<C: Controller> {
    // todo
    type Button: KeypadButton;

    fn find_shortest_cmd_seqs_between(
        &self,
        src: &Self::Button,
        dst: &Self::Button,
    ) -> impl Iterator<Item = Vec<C::Command>>;

    fn find_shortest_cmd_seqs_to_output(
        &self,
        output: &[Self::Button],
    ) -> Vec<Vec<C::Command>>;
} 

impl<K: Keypad> KeypadWithController<DirectionalKeypad> for K {
    type Button = K::Button;
    
    // XXX: Let's put `A` in the end of each shortest path.
    fn find_shortest_cmd_seqs_between(
        &self,
        src: &Self::Button,
        dst: &Self::Button,
    ) -> impl Iterator<Item = Vec<<DirectionalKeypad as Controller>::Command>> {
        let shortest_paths = self.find_shortest_paths_between(src, dst);

        shortest_paths.into_iter().map(|deltas| {
            let mut cmds = deltas.into_iter()
                .map(|delta| {
                    DirectionalKeypad::step_to_cmd(&delta)
                })
                .collect::<Vec<_>>();

            cmds.push(DirectionalKeypadButton::Activate);

            cmds
        })
    }

    fn find_shortest_cmd_seqs_to_output(
        &self,
        output: &[Self::Button],
    ) -> Vec<Vec<<DirectionalKeypad as Controller>::Command>> {
        let mut shortest_cmd_seqs = self.find_shortest_cmd_seqs_between(
            &Self::Button::default(),
            &output[0]
        )
        .collect::<Vec<_>>();

        for buttons in output.windows(2) {
            let suffixes = self.find_shortest_cmd_seqs_between(
                &buttons[0],
                &buttons[1],
            )
            .collect::<Vec<_>>();

            shortest_cmd_seqs = shortest_cmd_seqs.into_iter()
                .flat_map(|prefix| {
                    suffixes.iter()
                        .map(|suffix| {
                            let mut acc = prefix.clone();
                            acc.extend(suffix);
                            acc
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        shortest_cmd_seqs
    }
}

fn len_shortest_cmd_seqs_with_n_indirections_between(
    src: DirectionalKeypadButton,
    dst: DirectionalKeypadButton,
    num_additional_directional_keypads: usize,
    cache: &mut HashMap<
        (
            DirectionalKeypadButton,
            DirectionalKeypadButton,
            usize
        ),
        usize
    >,
) -> usize {
    if num_additional_directional_keypads == 1 {
        let len_shortest_cmd_seqs = DIRECTIONAL_KEYPAD
            .find_shortest_cmd_seqs_between(&src, &dst)
            .next().unwrap().len();

        cache.insert(
            (src, dst, num_additional_directional_keypads),
            len_shortest_cmd_seqs
        );

        return len_shortest_cmd_seqs;
    }

    if let Some(&len_shortest_cmd_seqs) = cache.get(&(
        src,
        dst,
        num_additional_directional_keypads,
    )) {
        return len_shortest_cmd_seqs;
    }

    let len_shortest_cmd_seqs = DIRECTIONAL_KEYPAD
        .find_shortest_cmd_seqs_between(&src, &dst)
        .map(|cmd_seqs| {
            let mut len_shortest_cmd_seqs =
                len_shortest_cmd_seqs_with_n_indirections_between(
                    DirectionalKeypadButton::Activate,
                    cmd_seqs[0],
                    num_additional_directional_keypads - 1,
                    cache);

            cmd_seqs.windows(2).for_each(|src_and_dst| {
                let src = src_and_dst[0].clone();
                let dst = src_and_dst[1].clone();

                len_shortest_cmd_seqs +=
                    len_shortest_cmd_seqs_with_n_indirections_between(
                        src,
                        dst,
                        num_additional_directional_keypads - 1,
                        cache);
            });

            len_shortest_cmd_seqs
        })
        .min()
        .unwrap();

    cache.insert(
        (src, dst, num_additional_directional_keypads),
        len_shortest_cmd_seqs,
    );

    len_shortest_cmd_seqs
}

fn len_shortest_cmd_seqs_with_n_indirections(
    cmd_seq: &[DirectionalKeypadButton],
    num_additional_directional_keypads: usize,
) -> usize {
    let mut cache = HashMap::<
        (
            DirectionalKeypadButton,
            DirectionalKeypadButton,
            usize,
        ),
        usize
    >::new();

    let mut len_shortest_cmd_seqs =
        len_shortest_cmd_seqs_with_n_indirections_between(
            DirectionalKeypadButton::Activate,
            cmd_seq[0],
            num_additional_directional_keypads,
            &mut cache);

    cmd_seq.windows(2).for_each(|src_and_dst| {
        let src = src_and_dst[0].clone();
        let dst = src_and_dst[1].clone();

        len_shortest_cmd_seqs +=
            len_shortest_cmd_seqs_with_n_indirections_between(
                src,
                dst,
                num_additional_directional_keypads,
                &mut cache);
    });

    len_shortest_cmd_seqs
}

fn main() {
    let num_and_terminal_cmd_seqs = INPUT.lines()
        .map(|line| {
            let n = *&line[0..(line.len() - 1)].parse::<usize>().unwrap();

            let target = line.chars()
                    .map(|c| NumericKeypadButton::try_from(c).unwrap())
                    .collect::<Vec<_>>();

            (n, NUMERIC_KEYPAD.find_shortest_cmd_seqs_to_output(&target))
        })
        .collect::<Vec<_>>();

    let part1 = num_and_terminal_cmd_seqs.iter()
        .map(|(n, cmd_seqs)| {
            let len_shortest_cmd_seqs = cmd_seqs.iter()
                .map(|cmd_seq| len_shortest_cmd_seqs_with_n_indirections(
                    cmd_seq,
                    2)
                )
                .min()
                .unwrap();

            n * len_shortest_cmd_seqs
        })
        .sum::<usize>();

    println!("{}", part1);

    let part2 = num_and_terminal_cmd_seqs.iter()
        .map(|(n, cmd_seqs)| {
            let len_shortest_cmd_seqs = cmd_seqs.iter()
                .map(|cmd_seq| len_shortest_cmd_seqs_with_n_indirections(
                    cmd_seq,
                    25)
                )
                .min()
                .unwrap();

            n * len_shortest_cmd_seqs
        })
        .sum::<usize>();

    println!("{}", part2);
}

#[cfg(test)]
mod test {
    use crate::{len_shortest_cmd_seqs_with_n_indirections, DirectionalKeypadButton, KeypadWithController, NumericKeypadButton, NUMERIC_KEYPAD};

    #[test]
    fn test_len_shortest_cmd_seqs_with_n_indirections() {
        let output = &[
            NumericKeypadButton::Num(0),
            NumericKeypadButton::Num(2),
            NumericKeypadButton::Num(9),
            NumericKeypadButton::Activate,
        ];

        let first_shotest_cmd_seqs = NUMERIC_KEYPAD 
            .find_shortest_cmd_seqs_to_output(output);

        assert_eq!(
            "<A^A>^^AvvvA".len(),
            first_shotest_cmd_seqs[0].len(),
        );

        let cmd_seq = &[
            DirectionalKeypadButton::Left,
            DirectionalKeypadButton::Activate,
            DirectionalKeypadButton::Up,
            DirectionalKeypadButton::Activate,
            DirectionalKeypadButton::Right,
            DirectionalKeypadButton::Up,
            DirectionalKeypadButton::Up,
            DirectionalKeypadButton::Activate,
            DirectionalKeypadButton::Down,
            DirectionalKeypadButton::Down,
            DirectionalKeypadButton::Down,
            DirectionalKeypadButton::Activate,
        ];

        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(),
            len_shortest_cmd_seqs_with_n_indirections(
                cmd_seq,
                1
            ),
        );

        assert_eq!(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
            len_shortest_cmd_seqs_with_n_indirections(
                cmd_seq,
                2
            ),
        );
    }
}