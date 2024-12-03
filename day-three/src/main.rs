use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let lines = parse_file("input.txt");
    let mut total = 0;
    for line in lines {
        let digits = parse_instances_of_mul_in_string(line);
        total += sum_products(&digits);
    }
    println!("{}", total)
}

// thanks stackoverflow
fn parse_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .collect()
}

fn parse_instances_of_mul_in_string(string: String) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"(?:mul\((\d+),(\d+)\))").unwrap();
    let digits: Vec<(i32, i32)> = pattern
        .captures_iter(string.as_str())
        .map(|caps| {
            let (_, [digit_one, digit_two]) = caps.extract();
            (
                digit_one.parse::<i32>().unwrap(),
                digit_two.parse::<i32>().unwrap(),
            )
        })
        .collect();

    return digits;
}

fn sum_products(digits: &Vec<(i32, i32)>) -> i32 {
    let mut total = 0;
    for (digit_one, digit_two) in digits {
        total += *digit_one * *digit_two
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test_one() {
        let result = parse_file("./mocks/mock1.txt");
        let want = vec!["testfilewithonelineshouldhaveonelineoutput".to_string()];
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_two() {
        let result = parse_file("./mocks/mock2.txt");
        let want = vec![
            "testfilewithonelineshouldhaveonelineoutput".to_string(),
            "nowthereisanotherlinesotwolinesoutput".to_string(),
        ];
        assert_eq!(want, result);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_one() {
        let string = "mul(2,4)extrawordsnothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(string);
        let result_length = result.len();
        let want = vec![(2, 4)];
        let want_length = 1;
        assert_eq!(want_length, result_length);
        assert_eq!(want, result);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_two() {
        let string = "mul(2,4)extrawordsmul(4,1)nothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(string);
        let result_length = result.len();
        let want = vec![(2, 4), (4, 1)];
        let want_length = 2;
        assert_eq!(want_length, result_length);
        assert_eq!(want, result);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_two_with_interference() {
        let string = "mul(2,4)extramul(2,2wordsmul(4,1)nothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(string);
        let result_length = result.len();
        let want = vec![(2, 4), (4, 1)];
        let want_length = 2;
        assert_eq!(want_length, result_length);
        assert_eq!(want, result);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_example() {
        let string =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = parse_instances_of_mul_in_string(string);
        let result_length = result.len();
        let want = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let want_length = 4;
        assert_eq!(want_length, result_length);
        assert_eq!(want, result);
    }

    #[test]
    fn sum_products_test_expect_4() {
        let digits = vec![(2, 2)];
        let result = sum_products(&digits);
        let want = 4;
        assert_eq!(want, result);
    }

    #[test]
    fn sum_products_test_expect_13() {
        let digits = vec![(2, 2), (3, 3)];
        let result = sum_products(&digits);
        let want = 13;
        assert_eq!(want, result);
    }

    #[test]
    fn sum_products_example() {
        let digits = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let result = sum_products(&digits);
        let want = 161;
        assert_eq!(want, result);
    }
}
