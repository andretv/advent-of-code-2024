use std::cmp::Ordering;

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
            if !validate_report(&report) {
                return None;
            }

            return Some(());
        })
        .count()
}

pub fn validate_report(report: &Vec<u32>) -> bool {
    let mut level_ordering = None;

    for (idx, cur) in report.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let prev = &report[idx - 1];

        match level_ordering {
            Some(ordering) => {
                let cmp = prev.cmp(cur);

                if cmp == Ordering::Equal || cmp != ordering {
                    return false;
                }
            }
            None => match prev.cmp(cur) {
                Ordering::Less => level_ordering = Some(Ordering::Less),
                Ordering::Greater => level_ordering = Some(Ordering::Greater),
                Ordering::Equal => return false,
            },
        }

        if prev.abs_diff(*cur) > 3 {
            return false;
        }
    }

    return true;
}
