use core::panic;

use advent_of_code_2024_utils::load_file;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    MarkedUp,
    MarkedDown,
    MarkedLeft,
    MarkedRight,
    Object,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
struct Guard {
    pub pos_x: usize,
    pub pos_y: usize,
    pub direction: Direction,
}

impl Guard {
    pub fn step(self, map: &[Vec<Cell>]) -> Self {
        match (self.direction, self.pos_x, self.pos_y) {
            (Direction::Up, pos_x, pos_y) if pos_y > 0 => Self {
                direction: match map[pos_y - 1][pos_x] {
                    Cell::Object => Direction::Right,
                    _ => Direction::Up,
                },
                pos_x,
                pos_y: match map[pos_y - 1][pos_x] {
                    Cell::Object => pos_y,
                    _ => pos_y - 1,
                },
            },
            (Direction::Down, pos_x, pos_y) if pos_y < map.len() - 1 => Self {
                direction: match map[pos_y + 1][pos_x] {
                    Cell::Object => Direction::Left,
                    _ => Direction::Down,
                },
                pos_x,
                pos_y: match map[pos_y + 1][pos_x] {
                    Cell::Object => pos_y,
                    _ => pos_y + 1,
                },
            },
            (Direction::Left, pos_x, pos_y) if pos_x > 0 => Self {
                direction: match map[pos_y][pos_x - 1] {
                    Cell::Object => Direction::Up,
                    _ => Direction::Left,
                },
                pos_x: match map[pos_y][pos_x - 1] {
                    Cell::Object => pos_x,
                    _ => pos_x - 1,
                },
                pos_y
            },
            (Direction::Right, pos_x, pos_y) if pos_x < map[0].len() - 1 => Self {
                direction: match map[pos_y][pos_x + 1] {
                    Cell::Object => Direction::Down,
                    _ => Direction::Right,
                },
                pos_x: match map[pos_y][pos_x + 1] {
                    Cell::Object => pos_x,
                    _ => pos_x + 1,
                },
                pos_y,
            },
            _ => self,
        }
    }
}

fn main() {
    let (guard, mut map) = load_file("day06/input.txt")
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => (None, Cell::Object),
                    '^' => (
                        Some(Guard {
                            pos_x: j,
                            pos_y: i,
                            direction: Direction::Up,
                        }),
                        Cell::MarkedUp,
                    ),
                    '>' => (
                        Some(Guard {
                            pos_x: j,
                            pos_y: i,
                            direction: Direction::Right,
                        }),
                        Cell::MarkedRight,
                    ),
                    '<' => (
                        Some(Guard {
                            pos_x: j,
                            pos_y: i,
                            direction: Direction::Left,
                        }),
                        Cell::MarkedLeft,
                    ),
                    'V' => (
                        Some(Guard {
                            pos_x: j,
                            pos_y: i,
                            direction: Direction::Down,
                        }),
                        Cell::MarkedDown,
                    ),
                    _ => (None, Cell::Empty),
                })
                .fold(
                    (None, Vec::new()),
                    |(guard, mut line), (cur_guard, cell)| {
                        line.push(cell);
                        (
                            match cur_guard {
                                None => guard,
                                Some(_) => cur_guard,
                            },
                            line,
                        )
                    },
                )
        })
        .fold((None, Vec::new()), |(guard, mut map), (cur_guard, line)| {
            map.push(line);
            (
                match cur_guard {
                    None => guard,
                    Some(_) => cur_guard,
                },
                map,
            )
        });

    let mut init = guard.unwrap();
    let mut fast = guard.unwrap().step(&map);
    let mut res_sln1 = 1;
    let mut res_sln2 = 0;
    while init != fast {
        init = init.step(&map);
        fast = fast.step(&map).step(&map);
        if let Cell::Empty =  map[init.pos_y][init.pos_x] {
            res_sln1 += 1;
        }
        map[init.pos_y][init.pos_x] = match (init.direction, map[init.pos_y][init.pos_x]) {
            (_, Cell::Object) => panic!("Rules wrong!"),
            (Direction:: Up, Cell::MarkedRight) => { res_sln2 += 1; Cell::MarkedUp },
            (Direction::Right, Cell::MarkedDown) => { res_sln2 += 1; Cell::MarkedRight },
            (Direction:: Down, Cell::MarkedLeft) => { res_sln2 += 1; Cell::MarkedDown },
            (Direction:: Left, Cell::MarkedUp) => { res_sln2 += 1; Cell::MarkedLeft },
            (Direction:: Up, _) => Cell::MarkedUp,
            (Direction::Right, _) => Cell::MarkedRight,
            (Direction:: Down, _) => Cell::MarkedDown,
            (Direction:: Left, _) => Cell::MarkedLeft,
        };
    }
    println!("sln1: {}", res_sln1);
    println!("sln2: {}", res_sln2);
}
