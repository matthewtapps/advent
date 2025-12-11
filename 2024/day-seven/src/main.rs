use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    println!("Hello, world!");
}

// thanks stackoverflow
fn parse_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    let lines: Vec<String> = buf
        .lines()
        .map(|line| line.expect("could not parse line"))
        .collect();
    return lines;
}

fn calculate_calibration_result(lines: &Vec<String>) -> i32 {
    unimplemented!()
}

fn parse_line(line: &String) -> Line {
    let mut split_line: Vec<&str> = line.split(":").collect();
    let target: i32 = split_line.remove(0).parse().unwrap();

    let mut children_line = split_line[0];

    // Get rid of the empty space at the start of the children
    children_line = &children_line[1..];

    let children: Vec<i32> = children_line
        .split(" ")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    return Line { target, children };
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    target: i32,
    children: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_acceptance() {
        let lines = parse_file("./mocks/example.txt");
        let result = calculate_calibration_result(&lines);
        let want = 3749;
        assert_eq!(want, result);
    }

    #[test]
    fn parse_line_test() {
        let line = "190: 10 19".to_string();
        let result = parse_line(&line);
        let want = Line {
            target: 190,
            children: vec![10, 19],
        };

        assert_eq!(want, result)
    }
}
