use std::path::Path;

const INPUT: &str = "input.txt";

fn parse_file(file_path: impl AsRef<Path>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let contents = std::fs::read_to_string(file_path).expect("no such file");

    let mut sections = contents.trim().split("\n\n");

    let ranges: Vec<(i64, i64)> = sections
        .next()
        .expect("missing ranges section")
        .split_whitespace()
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: i64 = parts[0].parse().expect("couldn't parse start value");
            let end: i64 = parts[1].parse().expect("couldn't parse end value");
            (start, end)
        })
        .collect();

    let values: Vec<i64> = sections
        .next()
        .expect("missing individuals section")
        .split_whitespace()
        .map(|num| num.parse().expect("couldn't parse individual number"))
        .collect();

    (ranges, values)
}

fn main() {
    let (ranges, values) = parse_file(INPUT);

    let mut fresh = 0;

    for value in &values {
        for (start, end) in &ranges {
            // if value is between start and end of a range, it's fresh
            if value >= start && value <= end {
                fresh += 1;
                // stop checking ranges to prevent double counts
                break;
            }
        }
    }

    println!("{}", fresh);
}
