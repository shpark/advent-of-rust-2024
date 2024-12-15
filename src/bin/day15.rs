const INPUT: &str = include_str!("./day15.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Box,
    BoxL,
    BoxR,
    Wall,
}

impl TryInto<Tile> for char {
    type Error = &'static str;

    fn try_into(self) -> Result<Tile, Self::Error> {
        if self == '.' {
            Ok(Tile::Empty)
        } else if self == 'O' {
            Ok(Tile::Box)
        } else if self == '[' {
            Ok(Tile::BoxL)
        } else if self == ']' {
            Ok(Tile::BoxR)
        } else if self == '#' {
            Ok(Tile::Wall)
        } else {
            Err("meheh")
        }
    }
}

impl TryFrom<Tile> for char {
    type Error = &'static str;

    fn try_from(tile: Tile) -> Result<Self, Self::Error> {
        match tile {
            Tile::Box => Ok('O'),
            Tile::BoxL => Ok('['),
            Tile::BoxR => Ok(']'),
            Tile::Empty => Ok('.'),
            Tile::Wall => Ok('#'),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            if let Ok(c) = self.clone().try_into() {
                c
            } else {
                '.'
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Command {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == '^' {
            Ok(Command::Up)
        } else if c == 'v' {
            Ok(Command::Down)
        } else if c == '<' {
            Ok(Command::Left)
        } else if c == '>' {
            Ok(Command::Right)
        } else {
            Err("meh")
        }
    }
}

impl Command {
    fn delta(&self) -> (i64, i64) {
        match self {
            Command::Up => (-1, 0),
            Command::Down => (1, 0),
            Command::Left => (0, -1),
            Command::Right => (0, 1),
        }
    }
}

struct Warehouse {
    robot: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

impl Warehouse {
    fn can_move_2x1_box(
        &self,
        (y, x): (usize, usize),
        command: &Command,
    ) -> bool {
        let (dy, dx) = command.delta();

        match (self.tiles[y][x], command) {
            (Tile::BoxL, Command::Left) | (Tile::BoxL, Command::Right) |
            (Tile::BoxR, Command::Left) | (Tile::BoxR, Command::Right) => {
                // Can move 2x1 box horizontally?
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                if self.tiles[y_next][x_next] == Tile::Wall {
                    return false;
                } else if self.can_move_2x1_box((y_next, x_next), command) {
                        return true;
                }

                false
            },
            (Tile::BoxL, Command::Up) | (Tile::BoxL, Command::Down) => {
                // Can move 2x1 box vertically?
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                if self.tiles[y_next][x_next] == Tile::Wall ||
                    self.tiles[y_next][x_next + 1] == Tile::Wall {
                    return false;
                } else if self.can_move_2x1_box((y_next, x_next), command) && 
                    self.can_move_2x1_box((y_next, x_next + 1), command) {
                        return true;
                }

                false
            },
            (Tile::BoxR, Command::Up) | (Tile::BoxR, Command::Down) => {
                // Can move 2x1 box vertically?
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                if self.tiles[y_next][x_next] == Tile::Wall ||
                    self.tiles[y_next][x_next - 1] == Tile::Wall {
                    return false;
                } else if self.can_move_2x1_box((y_next, x_next), command) && 
                    self.can_move_2x1_box((y_next, x_next - 1), command) {
                        return true;
                }

                false
            },
            (Tile::Empty, _) => {
                return true;
            },
            _ => panic!(),
        }
    }

    fn move_2x1_box(
        &mut self,
        (y, x): (usize, usize),
        command: &Command,
    ) {
        let (dy, dx) = command.delta();

        match (self.tiles[y][x], command) {
            (Tile::BoxL, Command::Left) | (Tile::BoxL, Command::Right) |
            (Tile::BoxR, Command::Left) | (Tile::BoxR, Command::Right) => {
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                self.move_2x1_box((y_next, x_next), command);

                self.tiles[y_next][x_next] = self.tiles[y][x];
                self.tiles[y][x] = Tile::Empty;
            },
            (Tile::BoxL, Command::Up) | (Tile::BoxL, Command::Down) => {
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                self.move_2x1_box((y_next, x_next), command);
                self.move_2x1_box((y_next, x_next + 1), command);
                self.tiles[y_next][x_next] = self.tiles[y][x];
                self.tiles[y_next][x_next + 1] = self.tiles[y][x + 1];
                self.tiles[y][x] = Tile::Empty;
                self.tiles[y][x + 1] = Tile::Empty;
            },
            (Tile::BoxR, Command::Up) | (Tile::BoxR, Command::Down) => {
                let (y_next, x_next) = (
                    (y as i64 + dy) as usize,
                    (x as i64 + dx) as usize,
                );

                self.move_2x1_box((y_next, x_next), command);
                self.move_2x1_box((y_next, x_next - 1), command);
                self.tiles[y_next][x_next] = self.tiles[y][x];
                self.tiles[y_next][x_next - 1] = self.tiles[y][x - 1];
                self.tiles[y][x] = Tile::Empty;
                self.tiles[y][x - 1] = Tile::Empty;
            },
            _ => {},
        }
    }

    fn step(&mut self, command: Command) {
        let (dy, dx) = command.delta();

        let (y_next, x_next) = (
            (self.robot.0 as i64 + dy) as usize,
            (self.robot.1 as i64 + dx) as usize
        );

        match (self.tiles[y_next][x_next], command) {
            (Tile::Empty, _) => {
                self.robot = (y_next, x_next);
            },
            (Tile::Box, _) => {
                // As the robot (@) attempts to move, if there are any boxes (O)
                // in the way, the robot will also attempt to push those boxes.
                //
                // E.g.,
                //      543210           43210
                //     (#.OOO@, [<]) -> (#OOO@., [])
                //
                // Approach:
                // - Find the first empty tile along the way
                //   - If a wall tile is encountered before an empty tile, it
                //     means the robot cannot push any boxes
                let mut i = 1;

                let first_empty_tile = loop {
                    let (y, x) = (
                        (self.robot.0 as i64 + i * dy) as usize,
                        (self.robot.1 as i64 + i * dx) as usize
                    );

                    match self.tiles[y][x] {
                        Tile::Wall => break None,
                        Tile::Empty => break Some((y, x)),
                        _ => { i += 1 },
                    }
                };

                if let Some((y, x)) = first_empty_tile {
                    self.tiles[y_next][x_next] = Tile::Empty;
                    self.robot = (y_next, x_next);
                    self.tiles[y][x] = Tile::Box;
                }
            },
            (Tile::BoxL, _) | (Tile::BoxR, _) => {
                if self.can_move_2x1_box((y_next, x_next), &command) {
                    self.move_2x1_box((y_next, x_next), &command);
                    self.robot = (y_next, x_next);
                }
            },
            _ => {},
        }
    }

    fn sum_gps_coordinates(&self) -> usize {
        let mut res = 0usize;

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if matches!(self.tiles[i][j], Tile::Box | Tile::BoxL) {
                    res += 100 * i + j;
                }
            }
        }

        res
    }

    fn expand(&mut self) {
        self.robot = (self.robot.0, self.robot.1 * 2);

        self.tiles = self.tiles.iter()
            .map(|row| {
                row.iter()
                    .flat_map(|tile| {
                        match tile {
                            Tile::Empty => vec![Tile::Empty, Tile::Empty],
                            Tile::Box => vec![Tile::BoxL, Tile::BoxR],
                            Tile::Wall => vec![Tile::Wall, Tile::Wall],
                            _ => panic!("ugh"),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }
}

impl TryFrom<&str> for Warehouse {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut num_boxes = 0u64;
        let mut num_walls = 0u64;

        let mut robot = (0usize, 0usize);

        let tiles = value.lines().enumerate()
            .map(|(i, line)| {
                line.chars().enumerate().map(|(j, c)| {
                    // update stats
                    if c == '#' {
                        num_walls += 1;
                    } else if c == 'O' {
                        num_boxes += 1;
                    }

                    if let Ok(tile) = c.try_into() {
                        tile
                    } else {
                        robot = (i, j);
                        Tile::Empty
                    }
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Warehouse {
            robot,
            tiles,
        })
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                write!(f, "{}", if self.robot == (i, j) {
                    '@'
                } else {
                    self.tiles[i][j].try_into().unwrap()
                })?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn main() {
    let parts = INPUT.split("\n\n").collect::<Vec<_>>();

    let mut warehouse: Warehouse = parts[0].try_into().unwrap();

    let attempts: Vec<Command> = parts[1].chars()
        .filter_map(|c| c.try_into().ok()).collect::<Vec<_>>();

    attempts.iter().for_each(|attempt| warehouse.step(*attempt));

    println!("{}", warehouse.sum_gps_coordinates());

    let mut warehouse: Warehouse = parts[0].try_into().unwrap();

    warehouse.expand();

    attempts.iter().for_each(|attempt| warehouse.step(*attempt));

    println!("{}", warehouse.sum_gps_coordinates());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{}", Tile::Box), "O");
    }

    #[test]
    fn test_parse_warehouse() {
        const WAREHOUSE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let warehouse: Result<Warehouse, _> = WAREHOUSE.try_into();

        assert!(matches!(warehouse, Ok(_)));

        let warehouse = warehouse.unwrap();

        assert_eq!(warehouse.robot, (2, 2));

        assert_eq!(
            format!("{}", warehouse),
            String::from("\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
")
        );
    }

    #[test]
    fn test_step() {
        const WAREHOUSE: &str = "\
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let mut warehouse: Warehouse = WAREHOUSE.try_into().unwrap();

        warehouse.step(Command::Right);

        assert_eq!(
            format!("{}", warehouse),
            String::from("\
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
")
        );
    }

    #[test]
    fn test_expand_and_move() {
        const WAREHOUSE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO@.O.O#
#..O...O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut warehouse: Warehouse = WAREHOUSE.try_into().unwrap();

        warehouse.expand();

        assert_eq!(
            format!("{}", warehouse),
            String::from("\
####################
##....[]....[]..[]##
##............[]..##
##..[][]@...[]..[]##
##....[]......[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
")
        );

        warehouse.step(Command::Left);
        warehouse.step(Command::Left);
        warehouse.step(Command::Left);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);
        warehouse.step(Command::Right);

        println!("{}", warehouse);

        assert_eq!(
            format!("{}", warehouse),
            String::from("\
####################
##....[]....[]..[]##
##............[]..##
##[][].......@[][]##
##....[]......[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
")
        );
    }

    #[test]
    fn test_vertical_push() {
        const WAREHOUSE: &str = "\
############
####[]....##
##[][][][]##
##...[][].##
##....@[].##
############
";

        let mut warehouse: Warehouse = WAREHOUSE.try_into().unwrap();

        warehouse.step(Command::Up);

        assert_eq!(
            format!("{}", warehouse),
            String::from("\
############
####[]....##
##[][][][]##
##...[][].##
##....@[].##
############
")
        );
    }
}
