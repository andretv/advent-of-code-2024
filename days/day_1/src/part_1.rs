pub fn solution(input: &str) -> u32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.lines().for_each(|line| {
        let mut chars = line.split_whitespace();
        let left = chars
            .next()
            .expect("Number character must be present.")
            .parse::<u32>()
            .expect("Character should be a number.");
        let right = chars
            .next()
            .expect("Number character must be present.")
            .parse::<u32>()
            .expect("Character should be a number.");

        left_list.push(left);
        right_list.push(right);
    });

    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(&right_list)
        .map(|(n1, n2)| n1.abs_diff(*n2))
        .sum()
}
