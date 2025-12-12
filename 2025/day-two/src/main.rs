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
        for number in start..end + 1 {
            // Treat as string so we can split in half
            let digits = number.to_string();

            let mid = digits.len() / 2;

            let left = &digits[..mid];
            let right = &digits[mid..];

            // If the halves match, it's invalid
            if left == right {
                println!("{}, {}", left, right);
                sum_invalids += number;
            }
        }
    }

    println!("{}", sum_invalids)
}
