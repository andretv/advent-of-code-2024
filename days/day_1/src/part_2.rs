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

    left_list
        .iter()
        .map(|n1| {
            let mut occurrences = 0;
            right_list.iter().for_each(|n2| {
                if n1 == n2 {
                    occurrences += 1;
                }
            });

            n1 * occurrences
        })
        .sum()
}
