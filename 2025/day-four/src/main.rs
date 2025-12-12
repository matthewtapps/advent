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
    let factory_map = parse_file(INPUT);
    let mut total = 0;

    for (y, line) in factory_map.iter().enumerate() {
        for (x, current_char) in line.chars().enumerate() {
            if current_char == '@' && count_adjacent_rolls(x, y, &factory_map) <= 3 {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

fn count_adjacent_rolls(x: usize, y: usize, factory_map: &[String]) -> usize {
    let mut sum: usize = 0;

    for (dx, dy) in NEIGHBOUR_INDICES {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && ny >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;

            if ny < factory_map.len()
                && nx < factory_map[ny].len()
                && factory_map[ny].chars().nth(nx).expect("out of bounds") == '@'
            {
                sum += 1
            }
        }
    }

    sum
}

const NEIGHBOUR_INDICES: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),

    (-1, 0),
    (1, 0),

    (-1, 1),
    (0, 1),
    (1, 1),
];
