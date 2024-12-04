const XMAS: &[char] = &['X', 'M', 'A', 'S'];

pub fn solution(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            let starting_dir = Direction::Right;
            let mut cur_dir = starting_dir.clone();

            loop {
                let mut cur_char = char;
                let mut cur_pos = (y, x);
                let mut xmas_idx = 0;
                while cur_char == &XMAS[xmas_idx] {
                    if XMAS.get(xmas_idx + 1).is_none() {
                        sum += 1;
                        break;
                    }

                    if let Some((next_char, next_pos)) = next(&input, cur_pos, &cur_dir) {
                        cur_char = next_char;
                        cur_pos = next_pos;
                        xmas_idx += 1;
                    } else {
                        break;
                    }
                }

                cur_dir = cur_dir.next_dir();
                if cur_dir == starting_dir {
                    break;
                }
            }
        });
    });

    sum
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn next_dir(&self) -> Direction {
        match self {
            Direction::Up => Direction::UpRight,
            Direction::UpRight => Direction::Right,
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::Down,
            Direction::Down => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
            Direction::UpLeft => Direction::Up,
        }
    }
}

fn next<'a>(
    input: &'a Vec<Vec<char>>,
    pos: (usize, usize),
    direction: &Direction,
) -> Option<(&'a char, (usize, usize))> {
    match direction {
        Direction::Up => {
            if pos.0.checked_sub(1) == None {
                return None;
            }
            if let Some(line) = input.get(pos.0 - 1) {
                return Some((&line[pos.1], (pos.0 - 1, pos.1)));
            }
            return None;
        }
        Direction::UpRight => {
            if pos.0.checked_sub(1) == None {
                return None;
            }
            if let Some(line) = input.get(pos.0 - 1) {
                if let Some(c) = line.get(pos.1 + 1) {
                    return Some((&c, (pos.0 - 1, pos.1 + 1)));
                }
                return None;
            }
            return None;
        }
        Direction::Right => {
            if let Some(c) = input[pos.0].get(pos.1 + 1) {
                return Some((&c, (pos.0, pos.1 + 1)));
            }
            return None;
        }
        Direction::DownRight => {
            if let Some(line) = input.get(pos.0 + 1) {
                if let Some(c) = line.get(pos.1 + 1) {
                    return Some((&c, (pos.0 + 1, pos.1 + 1)));
                }
                return None;
            }
            return None;
        }
        Direction::Down => {
            if let Some(line) = input.get(pos.0 + 1) {
                return Some((&line[pos.1], (pos.0 + 1, pos.1)));
            }
            return None;
        }
        Direction::DownLeft => {
            if let Some(line) = input.get(pos.0 + 1) {
                if pos.1.checked_sub(1) == None {
                    return None;
                }
                if let Some(c) = line.get(pos.1 - 1) {
                    return Some((&c, (pos.0 + 1, pos.1 - 1)));
                }
                return None;
            }
            return None;
        }
        Direction::Left => {
            if pos.1.checked_sub(1) == None {
                return None;
            }
            if let Some(c) = input[pos.0].get(pos.1 - 1) {
                return Some((&c, (pos.0, pos.1 - 1)));
            }
            return None;
        }
        Direction::UpLeft => {
            if pos.0.checked_sub(1) == None || pos.1.checked_sub(1) == None {
                return None;
            }
            if let Some(line) = input.get(pos.0 - 1) {
                if let Some(c) = line.get(pos.1 - 1) {
                    return Some((&c, (pos.0 - 1, pos.1 - 1)));
                }
                return None;
            }
            return None;
        }
    }
}
