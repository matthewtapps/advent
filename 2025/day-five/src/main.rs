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
    let (mut ranges, _values) = parse_file(INPUT);

    let mut fresh = 0;

    ranges.sort_by(|(a, _b), (c, _d)| a.cmp(c));

    let (mut current_start, mut current_end) = ranges[0];

    for (start, end) in ranges {
        // if the next range is continuous with the previous range,
        if start <= current_end + 1 {
            // combine them and continue
            current_end = current_end.max(end);
        } else {
            // otherwise, count the values between and add to the total
            fresh += current_end - current_start +1;
            // update current start and end to the new values
            current_start = start;
            current_end = end;
        }
    }

    // at the end, count the last range we were investigating
    fresh += current_end - current_start + 1;

    println!("{}", fresh);
}
