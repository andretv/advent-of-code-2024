use regex::Regex;

pub fn solution(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d+)\,(\d+)\)").expect("Regex should be valid.");

    input
        .lines()
        .map(|line| {
            regex
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
