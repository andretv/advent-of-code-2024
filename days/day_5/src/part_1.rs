#[derive(Debug)]
struct Rule {
    before: u8,
    after: u8,
}

pub fn solution(input: &str) -> u32 {
    let mut input_iter = input.lines();
    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Vec<u8>> = vec![];

    // Rules
    while let Some(line) = input_iter.next() {
        if line.is_empty() {
            break;
        }

        let split: Vec<&str> = line.split("|").collect();
        let before: u8 = split[0]
            .parse()
            .expect("Before needs to be a parsable number.");
        let after: u8 = split[1]
            .parse()
            .expect("After needs to be a parsable number.");

        rules.push(Rule { before, after });
    }

    // Updates
    while let Some(line) = input_iter.next() {
        let update: Vec<_> = line
            .split(",")
            .map(|n| {
                n.parse::<u8>()
                    .expect("Update character needs to be a parsable number.")
            })
            .collect();
        updates.push(update);
    }

    updates
        .iter()
        .filter_map(|update| {
            // Iter over each number.
            for (idx, number) in update.iter().enumerate() {
                // Checks the rules for that number.
                for rule in rules.iter().filter(|r| &r.before == number) {
                    // Checks every number before this number.
                    for i in 0..update[0..idx].len() {
                        if update[i] == rule.after {
                            return None;
                        }
                    }
                }
            }

            let middle_idx = ((update.len() + 1) / 2) - 1;
            return Some(u32::from(update[middle_idx]));
        })
        .sum()
}
