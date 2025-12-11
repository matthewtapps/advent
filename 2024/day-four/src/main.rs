use iota::iota;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let wordsearch = parse_file("./input.txt");
    println!("{}", count_xmas(&wordsearch));
    println!("{}", count_mas_crosses(&wordsearch));
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

fn count_xmas(wordsearch: &Vec<String>) -> i32 {
    let mut total = 0;
    for (y, line) in wordsearch.iter().enumerate() {
        for x in 0..line.len() {
            let coordinate = (i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
            total += count_xmas_for_coordinate(&wordsearch, &coordinate);
        }
    }

    return total;
}

// golang technique tings but make it a rust package lol
iota! {
    const X: i32 = 0 << iota;
        , M
        , A
        , S
}

fn count_xmas_for_coordinate(wordsearch: &Vec<String>, coordinate: &(i32, i32)) -> i32 {
    let mut total = 0;

    let bounds = get_maximum_bounds(&wordsearch);
    let current_letter = get_letter_for_coordinate(&coordinate, &wordsearch);

    for adjacent_coordinate in get_adjacent_coordinates(coordinate, &bounds) {
        let adjacent_letter = get_letter_for_coordinate(&adjacent_coordinate, &wordsearch);
        if current_letter == 0 && adjacent_letter == 1 {
            let next_coordinates =
                get_next_two_coordinates_in_direction(&coordinate, &adjacent_coordinate, &bounds);
            if next_two_coordinates_are_final_letters(&next_coordinates, &wordsearch) {
                total += 1
            }
        }
    }

    return total;
}

fn next_two_coordinates_are_final_letters(
    next_coordinates: &Vec<(i32, i32)>,
    wordsearch: &Vec<String>,
) -> bool {
    if next_coordinates.len() > 2 {
        panic!()
    }

    if next_coordinates.len() != 2 {
        return false;
    }

    let coordinate_one = next_coordinates[0];
    let coordinate_two = next_coordinates[1];

    let letter_one = get_letter_for_coordinate(&coordinate_one, &wordsearch);

    if letter_one != 2 {
        return false;
    }

    let letter_two = get_letter_for_coordinate(&coordinate_two, &wordsearch);

    if letter_two != 3 {
        return false;
    }

    return true;
}

fn get_adjacent_coordinates(
    coordinate: &(i32, i32),
    maximum_bounds: &(i32, i32),
) -> Vec<(i32, i32)> {
    let (x_coordinate, y_coordinate) = coordinate;
    let mut adjacent_coordinates: Vec<(i32, i32)> = Vec::new();

    for check_x in -1..2 {
        for check_y in -1..2 {
            let current_x = x_coordinate + check_x;
            let current_y = y_coordinate + check_y;
            let current_coordinate = (current_x, current_y);
            if current_coordinate != *coordinate
                && !coordinate_out_of_bounds(&current_coordinate, &maximum_bounds)
            {
                adjacent_coordinates.push(current_coordinate)
            }
        }
    }
    return adjacent_coordinates;
}

fn count_mas_crosses(wordsearch: &Vec<String>) -> i32 {
    let mut total = 0;
    for (y, line) in wordsearch.iter().enumerate() {
        for x in 0..line.len() {
            let coordinate = (i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
            if coordinate_is_mas_cross(&coordinate, &wordsearch) {
                total += 1
            }
        }
    }

    return total;
}

fn coordinate_is_mas_cross(coordinate: &(i32, i32), wordsearch: &Vec<String>) -> bool {
    let current_coordinate_letter = get_letter_for_coordinate(&coordinate, &wordsearch);
    if current_coordinate_letter != 2 {
        return false;
    }

    let bounds = get_maximum_bounds(&wordsearch);
    let adjacent_coordinates = get_cross_coordinates_around_coordinate(&coordinate, &bounds);

    if adjacent_coordinates.len() != 4 {
        return false;
    }

    let mut adacent_letters = Vec::new();

    for adjacent_coordinate in adjacent_coordinates {
        adacent_letters.push(get_letter_for_coordinate(&adjacent_coordinate, &wordsearch))
    }

    let valid_patterns = vec![
        vec![3, 1, 3, 1],
        vec![1, 3, 1, 3],
        vec![1, 1, 3, 3],
        vec![3, 3, 1, 1],
    ];

    return valid_patterns.contains(&adacent_letters);
}

fn get_cross_coordinates_around_coordinate(
    coordinate: &(i32, i32),
    bounds: &(i32, i32),
) -> Vec<(i32, i32)> {
    let (x_coordinate, y_coordinate) = coordinate;
    let mut adjacent_coordinates: Vec<(i32, i32)> = Vec::new();
    let check_coordinates = vec![
        (x_coordinate - 1, y_coordinate - 1),
        (x_coordinate + 1, y_coordinate - 1),
        (x_coordinate - 1, y_coordinate + 1),
        (x_coordinate + 1, y_coordinate + 1),
    ];

    for check in check_coordinates {
        if !coordinate_out_of_bounds(&check, &bounds) {
            adjacent_coordinates.push(check)
        }
    }

    return adjacent_coordinates;
}

fn get_next_two_coordinates_in_direction(
    coordinate: &(i32, i32),
    correct_letter_coordinate: &(i32, i32),
    maximum_bounds: &(i32, i32),
) -> Vec<(i32, i32)> {
    let direction = get_direction_from_coordinates(&coordinate, &correct_letter_coordinate);
    let (x, y) = &correct_letter_coordinate;

    let next_coordinates = match direction {
        0 => vec![(x - 1, y - 1), (x - 2, y - 2)],
        1 => vec![(*x, y - 1), (*x, y - 2)],
        2 => vec![(x + 1, y - 1), (x + 2, y - 2)],
        3 => vec![(x - 1, *y), (x - 2, *y)],
        4 => vec![(x + 1, *y), (x + 2, *y)],
        5 => vec![(x - 1, y + 1), (x - 2, y + 2)],
        6 => vec![(*x, y + 1), (*x, y + 2)],
        7 => vec![(x + 1, y + 1), (x + 2, y + 2)],
        _ => panic!(),
    };

    for coordinate in &next_coordinates {
        if coordinate_out_of_bounds(&coordinate, &maximum_bounds) {
            return Vec::new();
        }
    }

    return next_coordinates;
}

fn get_direction_from_coordinates(
    coordinate: &(i32, i32),
    correct_letter_coordinate: &(i32, i32),
) -> i32 {
    let (x, y) = &coordinate;
    let (next_x, next_y) = &correct_letter_coordinate;
    let check_x = next_x - x;
    let check_y = next_y - y;

    match (check_x, check_y) {
        (-1, -1) => 0,
        (0, -1) => 1,
        (1, -1) => 2,
        (-1, 0) => 3,
        (0, 0) => panic!(),
        (1, 0) => 4,
        (-1, 1) => 5,
        (0, 1) => 6,
        (1, 1) => 7,
        _ => panic!(),
    }
}

fn get_letter_for_coordinate(coordinate: &(i32, i32), wordsearch: &Vec<String>) -> i32 {
    let slice_coordinate = get_slice_coordinate(&coordinate);
    return get_letter_for_slice_coordinate(&slice_coordinate, &wordsearch);
}

fn get_slice_coordinate(coordinate: &(i32, i32)) -> (usize, usize) {
    let (x, y) = coordinate;
    let x_usize = usize::try_from(*x).unwrap();
    let y_usize = usize::try_from(*y).unwrap();
    return (x_usize, y_usize);
}

fn get_letter_for_slice_coordinate(
    slice_coordinate: &(usize, usize),
    wordsearch: &Vec<String>,
) -> i32 {
    let (slice_x, slice_y) = slice_coordinate;
    return convert_letter_to_int(&wordsearch[*slice_y][*slice_x..*slice_x + 1]);
}

fn convert_letter_to_int(letter: &str) -> i32 {
    match letter {
        "X" => 0,
        "M" => 1,
        "A" => 2,
        "S" => 3,
        _ => {
            panic!();
        }
    }
}

fn get_maximum_bounds(wordsearch: &Vec<String>) -> (i32, i32) {
    let maximum_x = i32::try_from(wordsearch[0].len()).unwrap() - 1;
    let maximum_y = i32::try_from(wordsearch.len()).unwrap() - 1;
    return (maximum_x, maximum_y);
}

fn coordinate_out_of_bounds(coordinate: &(i32, i32), maximum_bounds: &(i32, i32)) -> bool {
    let (maximum_x, maximum_y) = *maximum_bounds;
    let (x, y) = *coordinate;
    return x > maximum_x || y > maximum_y || x < 0 || y < 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test_one() {
        let result = parse_file("./mocks/mock1.txt");
        let want = vec!["MMSXXMASM".to_string()];
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_two() {
        let result = parse_file("./mocks/mock2.txt");
        let want = vec!["MMSXXMASM".to_string(), "MSAMXMSMS".to_string()];
        assert_eq!(want, result);
    }

    #[test]
    fn part_one_acceptance_test() {
        let wordsearch = parse_file("./mocks/example.txt");
        let result = count_xmas(&wordsearch);
        let want = 18;
        assert_eq!(want, result);
    }

    #[test]
    fn count_xmas_test_expect_one() {
        let wordsearch = vec!["XMAS".to_string()];
        let result = count_xmas(&wordsearch);
        let want = 1;
        assert_eq!(want, result);
    }

    #[test]
    fn count_xmas_expect_six() {
        let wordsearch = vec![
            // Horizontal and diagonal
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let result = count_xmas(&wordsearch);
        let want = 6;
        assert_eq!(want, result);
    }

    #[test]
    fn count_xmas_expect_twelve() {
        let wordsearch = vec![
            "XMASXMAS".to_string(),
            "XMASXMAS".to_string(),
            "XMASXMAS".to_string(),
            "XMASXMAS".to_string(),
        ];
        let result = count_xmas(&wordsearch);
        let want = 12;
        assert_eq!(want, result);
    }

    #[test]
    fn count_xmas_for_coordinate_expect_one() {
        let wordsearch = vec!["XMAS".to_string()];
        let coordinate = (0, 0);
        let result = count_xmas_for_coordinate(&wordsearch, &coordinate);
        let want = 1;
        assert_eq!(want, result);
    }

    #[test]
    fn count_xmas_for_coordinate_expect_two() {
        let wordsearch = vec![
            // Horizontal and diagonal
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let coordinate = (0, 0);
        let result = count_xmas_for_coordinate(&wordsearch, &coordinate);
        let want = 2;
        assert_eq!(want, result);
    }
    #[test]
    fn get_adjacent_coordinates_test_one() {
        let coordinate: (i32, i32) = (0, 0);
        let result = get_adjacent_coordinates(&coordinate, &(3, 3));
        let want: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (1, 1)];
        assert_eq!(want, result);
    }

    #[test]
    fn coordinate_out_of_bounds_test_expect_true_one() {
        let coordinate = (-1, 0);
        let maximum_bounds = (3, 3);
        let result = coordinate_out_of_bounds(&coordinate, &maximum_bounds);
        let want = true;
        assert_eq!(want, result)
    }

    #[test]
    fn coordinate_out_of_bounds_test_expect_true_two() {
        let coordinate = (3, 4);
        let maximum_bounds = (3, 3);
        let result = coordinate_out_of_bounds(&coordinate, &maximum_bounds);
        let want = true;
        assert_eq!(want, result)
    }

    #[test]
    fn coordinate_out_of_bounds_test_expect_false_one() {
        let coordinate = (0, 0);
        let maximum_bounds = (3, 3);
        let result = coordinate_out_of_bounds(&coordinate, &maximum_bounds);
        let want = false;
        assert_eq!(want, result)
    }

    #[test]
    fn coordinate_out_of_bounds_test_expect_false_two() {
        let coordinate = (3, 3);
        let maximum_bounds = (3, 3);
        let result = coordinate_out_of_bounds(&coordinate, &maximum_bounds);
        let want = false;
        assert_eq!(want, result)
    }

    #[test]
    fn get_maximum_bounds_test() {
        let wordsearch = vec![
            // Horizontal and diagonal
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let result = get_maximum_bounds(&wordsearch);
        let want = (3, 3);
        assert_eq!(want, result)
    }

    #[test]
    fn convert_letter_to_int_test() {
        let letter = "A";
        let result = convert_letter_to_int(&letter);
        let want = 2;
        assert_eq!(want, result)
    }

    #[test]
    fn get_slice_coordinate_test() {
        let coordinate = (2, 1);
        let result = get_slice_coordinate(&coordinate);
        let want = (2, 1);
        assert_eq!(want, result)
    }

    #[test]
    fn get_letter_for_slice_coordinate_test() {
        let wordsearch = vec![
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let slice_coordinate = (2, 1);
        let result = get_letter_for_slice_coordinate(&slice_coordinate, &wordsearch);
        let want = 2;
        assert_eq!(want, result)
    }

    #[test]
    fn get_letter_for_coordinate_test() {
        let wordsearch = vec![
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let coordinate = (2, 3);
        let result = get_letter_for_coordinate(&coordinate, &wordsearch);
        let want = 2;
        assert_eq!(want, result)
    }

    #[test]
    fn get_direction_from_coordinates_test() {
        let coordinate = (1, 1);
        let next_coordinate = (1, 2);
        let result = get_direction_from_coordinates(&coordinate, &next_coordinate);
        let want = 6;
        assert_eq!(want, result)
    }

    #[test]
    fn get_next_two_coordinates_in_direction_test() {
        let coordinate = (1, 1);
        let next_coordinate = (1, 2);
        let bounds = (4, 4);
        let result = get_next_two_coordinates_in_direction(&coordinate, &next_coordinate, &bounds);
        let want = vec![(1, 3), (1, 4)];
        assert_eq!(want, result)
    }

    #[test]
    fn get_cross_coordinates_around_coordinate_test() {
        let coordinate = (1, 1);
        let bounds = (4, 4);
        let result = get_cross_coordinates_around_coordinate(&coordinate, &bounds);
        let want = vec![(0, 0), (2, 0), (0, 2), (2, 2)];
        assert_eq!(want, result)
    }

    #[test]
    fn coordinate_is_mas_cross_test() {
        let wordsearch = vec![
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
            "XMAS".to_string(),
        ];
        let coordinate = (2, 2);
        let result = coordinate_is_mas_cross(&coordinate, &wordsearch);
        let want = true;
        assert_eq!(want, result)
    }

    #[test]
    fn part_two_acceptance_test() {
        let wordsearch = parse_file("./mocks/example.txt");
        let result = count_mas_crosses(&wordsearch);
        let want = 9;
        assert_eq!(want, result);
    }
}
