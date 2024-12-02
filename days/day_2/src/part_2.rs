pub fn solution(input: &str) -> usize {
    input
        .lines()
        // Parse each line into a `Vec<u32>`
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse::<u32>().expect("Character to be a number."))
                .collect::<Vec<u32>>()
        })
        .filter_map(|report| {
            if !crate::part_1::validate_report(&report) {
                return problem_dampener(&report);
            }

            return Some(());
        })
        .count()
}

fn problem_dampener(report: &Vec<u32>) -> Option<()> {
    for idx in 0..report.len() {
        let mut temp_report = report.clone();
        temp_report.remove(idx);

        if crate::part_1::validate_report(&temp_report) {
            return Some(());
        }
    }

    None
}
