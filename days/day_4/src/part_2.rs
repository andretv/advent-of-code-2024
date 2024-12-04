const M: &char = &'M';
const A: &char = &'A';
const S: &char = &'S';

pub fn solution(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            if char == A {
                if check_for_xmas(&input, (y, x)) {
                    sum += 1;
                }
            }
        });
    });

    sum
}

fn check_for_xmas(input: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0.checked_sub(1).is_none() || pos.1.checked_sub(1).is_none() {
        return false;
    }

    if input.get(pos.0 - 1).is_none()
        || input.get(pos.0 + 1).is_none()
        || input[pos.0].get(pos.1 - 1).is_none()
        || input[pos.0].get(pos.1 + 1).is_none()
    {
        return false;
    };

    let top_left = &input[pos.0 - 1][pos.1 - 1];
    let top_right = &input[pos.0 - 1][pos.1 + 1];
    let bottom_left = &input[pos.0 + 1][pos.1 - 1];
    let bottom_right = &input[pos.0 + 1][pos.1 + 1];

    if (top_left == M && bottom_right == S || top_left == S && bottom_right == M)
        && (top_right == M && bottom_left == S || top_right == S && bottom_left == M)
    {
        return true;
    }

    return false;
}
