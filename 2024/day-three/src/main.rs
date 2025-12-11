use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use fancy_regex::Regex;

fn main() {
    let line = parse_file("input.txt");
    let mut total = 0;
    let digits = parse_instances_of_mul_in_string(&line);
    total += sum_products(&digits);
    println!("{}", total);

    total = calculate_conditional_result(line);
    println!("{}", total)
}

fn calculate_conditional_result(line: String) -> i32 {
    let mut total = 0;
    let mut valid = true;
    let mul_instances = capture_mul_and_prefix(&line);
    for instance in mul_instances {
        let dos_or_donts = capture_do_or_dont(&instance);
        let length = dos_or_donts.len();
        let last_prefix = last_prefix_is_do(&dos_or_donts);
        if valid {
            if length == 0 || last_prefix {
                let digits = parse_instances_of_mul_in_string(&instance);
                total += sum_products(&digits);
            } else {
                valid = false
            }
        } else {
            if last_prefix {
                valid = true;
                let digits = parse_instances_of_mul_in_string(&instance);
                total += sum_products(&digits);
            }
        }
        println!("{instance}, {valid}");
    }
    return total;
}

// thanks stackoverflow
fn parse_file(file_path: impl AsRef<Path>) -> String {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    let mut full_string = "".to_string();
    let lines: Vec<String> = buf
        .lines()
        .map(|line| line.expect("could not parse line"))
        .collect();
    for line in lines {
        full_string.push_str(line.as_str())
    }
    return full_string;
}

fn parse_instances_of_mul_in_string(string: &String) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"(?:mul\((?<one>\d+),(?<two>\d+)\))").unwrap();
    let digits: Vec<(i32, i32)> = pattern
        .captures_iter(string.as_str())
        .map(|caps| {
            let result = caps.unwrap();
            let digit_one = result.name("one").unwrap().as_str();
            let digit_two = result.name("two").unwrap().as_str();
            (
                digit_one.parse::<i32>().unwrap(),
                digit_two.parse::<i32>().unwrap(),
            )
        })
        .collect();

    return digits;
}

fn capture_mul_and_prefix(string: &String) -> Vec<String> {
    let pattern = Regex::new(r"(?<mul_with_prefix>\G.*?mul\(\d+,\d+\))+").unwrap();
    let muls_with_prefixes: Vec<String> = pattern
        .captures_iter(string.as_str())
        .map(|caps| {
            caps.unwrap()
                .name("mul_with_prefix")
                .unwrap()
                .as_str()
                .to_string()
        })
        .collect();
    return muls_with_prefixes;
}

fn capture_do_or_dont(string: &String) -> Vec<String> {
    let pattern = Regex::new(r"(?<do_or_dont>do\(\)|don't\(\))").unwrap();
    let dos_or_donts = pattern
        .captures_iter(string.as_str())
        .map(|caps| {
            caps.unwrap()
                .name("do_or_dont")
                .unwrap()
                .as_str()
                .to_string()
        })
        .collect();
    return dos_or_donts;
}

fn last_prefix_is_do(strings: &Vec<String>) -> bool {
    if strings.len() == 0 {
        return false;
    }
    let last_prefix = strings.last().unwrap();
    if last_prefix == &"do()".to_string() {
        return true;
    }
    return false;
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
        let want = "testfilewithonelineshouldhaveonelineoutput".to_string();
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_two() {
        let result = parse_file("./mocks/mock2.txt");
        let want =
            "testfilewithonelineshouldhaveonelineoutputnowthereisanotherlinesotwolinesoutput"
                .to_string();
        assert_eq!(want, result);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_one() {
        let string = "mul(2,4)extrawordsnothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(&string);
        let result_length = result.len();
        let want = vec![(2, 4)];
        let want_length = 1;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_two() {
        let string = "mul(2,4)extrawordsmul(4,1)nothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(&string);
        let result_length = result.len();
        let want = vec![(2, 4), (4, 1)];
        let want_length = 2;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_expect_two_with_interference() {
        let string = "mul(2,4)extramul(2,2wordsmul(4,1)nothingmuch".to_string();
        let result = parse_instances_of_mul_in_string(&string);
        let result_length = result.len();
        let want = vec![(2, 4), (4, 1)];
        let want_length = 2;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn parse_instances_of_mul_in_string_test_example() {
        let string =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = parse_instances_of_mul_in_string(&string);
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

    #[test]
    fn capture_mul_and_prefix_test_expect_1() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_".to_string();
        let result = capture_mul_and_prefix(&string);
        let result_length = result.len();
        let want = vec!["xmul(2,4)".to_string()];
        let want_length = 1;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn capture_mul_and_prefix_test_expect_2() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)".to_string();
        let result = capture_mul_and_prefix(&string);
        let result_length = result.len();
        let want = vec![
            "xmul(2,4)".to_string(),
            "%&mul[3,7]!@^do_not_mul(5,5)".to_string(),
        ];
        let want_length = 2;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn capture_do_or_dont_test_expect_2() {
        let string = "%&mul[3,7]!@^do()_notdon't()_mul(5,5)".to_string();
        let result = capture_do_or_dont(&string);
        let result_length = result.len();
        let want = vec!["do()".to_string(), "don't()".to_string()];
        let want_length = 2;
        assert_eq!(want, result);
        assert_eq!(want_length, result_length);
    }

    #[test]
    fn last_prefix_is_do_test_expect_true() {
        let strings = vec!["don't()".to_string(), "do()".to_string()];
        let result = last_prefix_is_do(&strings);
        let want = true;
        assert_eq!(want, result);
    }

    #[test]
    fn last_prefix_is_do_test_expect_false() {
        let strings = vec!["do()".to_string(), "don't()".to_string()];
        let result = last_prefix_is_do(&strings);
        let want = false;
        assert_eq!(want, result);
    }
}
