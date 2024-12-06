#[derive(Debug)]
struct Cell {
    obstructed: bool,
    visited: bool,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_dir(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn solution(input: &str) -> u32 {
    let mut sum = 0;
    let mut pos: (usize, usize) = (0, 0);
    let mut dir: Direction = Direction::Right;

    let mut input: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Cell {
                        obstructed: true,
                        visited: false,
                    },
                    '>' => {
                        pos = (y, x);
                        dir = Direction::Right;

                        Cell {
                            obstructed: false,
                            visited: false,
                        }
                    }
                    '<' => {
                        pos = (y, x);
                        dir = Direction::Left;

                        Cell {
                            obstructed: false,
                            visited: false,
                        }
                    }
                    '^' => {
                        pos = (y, x);
                        dir = Direction::Up;

                        Cell {
                            obstructed: false,
                            visited: false,
                        }
                    }
                    'v' => {
                        pos = (y, x);
                        dir = Direction::Down;

                        Cell {
                            obstructed: false,
                            visited: false,
                        }
                    }
                    _ => Cell {
                        obstructed: false,
                        visited: false,
                    },
                })
                .collect()
        })
        .collect();

    loop {
        if !input[pos.0][pos.1].visited {
            sum += 1;
            input[pos.0][pos.1].visited = true;
        }

        match dir {
            Direction::Up => {
                if pos.0.checked_sub(1).is_none() {
                    break;
                }
                if input[pos.0 - 1][pos.1].obstructed {
                    dir = dir.next_dir();
                    continue;
                }
                pos = (pos.0 - 1, pos.1);
            }
            Direction::Right => {
                if pos.1 >= input[pos.0].len() {
                    break;
                }
                if input[pos.0][pos.1 + 1].obstructed {
                    dir = dir.next_dir();
                    continue;
                }
                pos = (pos.0, pos.1 + 1);
            }
            Direction::Down => {
                if pos.0 + 1 >= input.len() {
                    break;
                }
                if input[pos.0 + 1][pos.1].obstructed {
                    dir = dir.next_dir();
                    continue;
                }
                pos = (pos.0 + 1, pos.1);
            }
            Direction::Left => {
                if pos.1.checked_sub(1).is_none() {
                    break;
                }
                if input[pos.0][pos.1 - 1].obstructed {
                    dir = dir.next_dir();
                    continue;
                }
                pos = (pos.0, pos.1 - 1);
            }
        }
    }

    sum
}
