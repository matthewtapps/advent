use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const INPUT: &str = "mocks/mock1.txt";

fn parse_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .collect()
}

fn main() {
    let inputs = parse_file(INPUT);
    let mut sum: i64 = 0;

    for line in inputs {
        // Turn the line into a vec of u32s
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let length = digits.len();
        // Result will be a 12-digit array of u32s
        let mut result = [0u32; 12];
        // Start looking at index 0
        let mut search_start = 0;

        // current is the index of the result digit we're currently trying to fill
        for current in 0..12 {
            // We need at least 12 digits in a row
            let search_end = length - (11 - current);

            let mut best_value = 0;
            let mut best_index = search_start;

            for i in search_start..search_end {
                // If the current digit is larger than our current best value,
                // update the best value and best index
                if digits[i] > best_value {
                    best_value = digits[i];
                    best_index = i;
                }
            }

            // Set our result for the current index to the best value we found,
            // and update our search start to be the index after our best index
            result[current] = best_value;
            search_start = best_index + 1;
        }

        sum += result.iter().fold(0i64, |acc, &d| acc * 10 + d as i64);
    }

    println!("{}", sum);
}
