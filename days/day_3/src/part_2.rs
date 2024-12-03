use regex::Regex;
use std::sync::LazyLock;

const MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+)\,(\d+)\)").expect("Regex should be valid."));
const DO: &str = "do()";
const DONT: &str = "don't()";

pub fn solution(input: &str) -> u32 {
    let mut sum = 0;
    let mut cursor: usize = 0;
    let mut should_mul = true;

    loop {
        match should_mul {
            true => {
                let next_dont_idx = input[cursor..].find(DONT);
                if let None = next_dont_idx {
                    sum += calculate_muls(&input[cursor..]);
                    return sum;
                }
                let next_dont_idx = next_dont_idx.unwrap();
                sum += calculate_muls(&input[cursor..cursor + next_dont_idx]);
                cursor += next_dont_idx;
                should_mul = false;
            }
            false => {
                let next_do_idx = input[cursor..].find(DO);
                if let None = next_do_idx {
                    return sum;
                }
                let next_do_idx = next_do_idx.unwrap();
                cursor += next_do_idx;
                should_mul = true;
            }
        };
    }
}

fn calculate_muls(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            MUL_REGEX
                .captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [v1, v2])| {
                    v1.parse::<u32>()
                        .expect("Character sequence to be a number.")
                        * v2.parse::<u32>()
                            .expect("Character sequence to be a number.")
                })
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
}
