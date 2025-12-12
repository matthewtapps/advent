use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

enum Direction {
    Left,
    Right,
}

const INPUT: &str = "input.txt";

fn parse_file(file_path: impl AsRef<Path>) -> Vec<(Direction, i32)> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    buf.lines()
        .map(|line| {
            let line = line.expect("could not parse line");
            let direction = match line.chars().next() {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!("invalid direction"),
            };
            let distance: i32 = line[1..].parse().expect("couldn't parse distance");
            (direction, distance)
        })
        .collect()
}

fn main() {
    let inputs = parse_file(INPUT);

    let mut dial = 50;
    let mut landings = 0;
    let mut crossings = 0;

    for (direction, distance) in inputs {
        // How many times did we cross 0
        if distance >= 100 {
            crossings += distance / 100;
        }

        // Net movement is modulo 100
        let movement = distance % 100;

        // Track start to know if it was 0 later
        let start = dial;

        match direction {
            Direction::Left => dial -= movement,
            Direction::Right => dial += movement,
        }

        // Did we cross forwards?
        if dial > 100 {
            crossings += 1;
        }

        // Did we cross backwards without starting on 0?
        if start != 0 && dial < 0 {
            crossings += 1;
        }

        // Calculate new dial
        dial %= 100;
        if dial < 0 {
            dial += 100;
        }

        // Track landings
        if dial == 0 {
            landings += 1;
        }
    }

    println!("{}, {}, {}", crossings, landings, crossings + landings)
}
