use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const INPUT: &str = "input.txt";

fn parse_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .collect()
}

fn main() {
    let inputs = parse_file(INPUT);
    let mut sum = 0;

    for line in inputs {
        let mut left = 0;
        let mut right = 0;
        let length = line.len();
        for index in 0..length {
            let digit = line
                .chars()
                .nth(index)
                .expect("couldn't get char index")
                .to_string()
                .parse()
                .expect("couldn't parse char");

            // Replace the left digit with the new highest, if we have at
            // least one more digit remaining to take the place of the right digit
            if digit > left && index < length - 1 {
                left = digit;
                right = 0;
                continue;
            }

            // Take the highest possible right digit
            if digit > right {
                right = digit;
            }
        }

        sum += (left.to_string() + &right.to_string()).parse::<i32>().expect("couldn't parse final number");

        println!("{} {}", left, right);
    }

    println!("{}", sum);
}

