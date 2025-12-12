use std::path::Path;

const INPUT: &str = "input.txt";

fn parse_file(file_path: impl AsRef<Path>) -> Vec<(i64, i64)> {
    let contents = std::fs::read_to_string(file_path).expect("no such file");
    contents
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();

            let start: i64 = parts[0].parse().expect("couldn't parse start value");
            let end: i64 = parts[1].parse().expect("couldn't parse end value");

            (start, end)
        })
        .collect()
}

fn main() {
    let inputs = parse_file(INPUT);

    let mut sum_invalids = 0;

    for (start, end) in inputs {
        for number in start..=end {
            // Treat as string so we can do slices
            let digits = number.to_string();
            // Get the length for indexing slices
            let length = digits.len();

            for pattern_len in 1..=length / 2 {
                // If the modulo isn't 0, the pattern can't repeat
                if length % pattern_len != 0 {
                    continue;
                }

                // Get the current slice as the pattern
                let pattern = &digits[..pattern_len];

                // Check if the digits is entirely made up of this slice repeating
                if is_all_repititions(&digits, pattern) {
                    sum_invalids += number;
                    break;
                }
            }
        }
    }

    println!("{}", sum_invalids)
}

fn is_all_repititions(string: &str, pattern: &str) -> bool {
    // Base case: If string is empty, we've matched everything
    if string.is_empty() {
        return true;
    }

    // Recursive case: If string starts with the pattern, recursively check the
    // rest of the string
    if let Some(remainder) = string.strip_prefix(pattern) {
        return is_all_repititions(remainder, pattern);
    }

    // If the string didn't start with the pattern, it's not all reptitions
    false
}
