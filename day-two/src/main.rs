use std::fs;

fn main() {
    let lists = parse_file("input.txt");
    println!("{}", count_safe_reports(lists));
}

fn count_safe_reports(lists: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for list in lists {
        if step_size_safe(&list) && report_direction_safe(&list) {
            safe_reports += 1
        }
    }

    return safe_reports;
}

fn step_size_safe(list: &Vec<i32>) -> bool {
    for (index, _) in list.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let step_size = (list[index] - list[index - 1]).abs();

        if step_size > 3 {
            return false;
        }
    }
    return true;
}

#[derive(PartialEq, Eq)]
enum Direction {
    Ascending,
    Descending,
}

fn report_direction_safe(list: &Vec<i32>) -> bool {
    let mut direction = Direction::Ascending;
    for (index, _) in list.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let comparison = list[index] > list[index - 1];
        let equal = list[index] == list[index - 1];

        if equal {
            return false;
        }

        if index == 1 {
            direction = if comparison {
                Direction::Ascending
            } else {
                Direction::Descending
            };
            continue;
        }

        let current_direction = if comparison {
            Direction::Ascending
        } else {
            Direction::Descending
        };

        if current_direction != direction {
            return false;
        }
    }
    return true;
}

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).unwrap();
    let mut lists: Vec<Vec<i32>> = Vec::new();

    for line in contents.split("\n") {
        let mut list: Vec<i32> = Vec::new();

        if line.len() == 0 {
            continue;
        };

        for number in line.split_whitespace() {
            let integer: i32 = number.parse().unwrap();
            list.push(integer)
        }

        lists.push(list)
    }
    return lists;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test_one() {
        let result = parse_file("./mocks/mock1.txt");
        let want = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_two() {
        let result = parse_file("./mocks/mock2.txt");
        let want = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]];
        assert_eq!(want, result);
    }

    #[test]
    fn step_size_safe_test_expect_safe() {
        let list = vec![1, 2, 3, 4, 5];
        let result = step_size_safe(&list);
        let want = true;
        assert_eq!(want, result);
    }

    #[test]
    fn step_size_safe_test_expect_unsafe() {
        let list = vec![1, 2, 3, 7, 8];
        let result = step_size_safe(&list);
        let want = false;
        assert_eq!(want, result);
    }

    #[test]
    fn count_safe_reports_test_expect_1() {
        let lists = vec![vec![1, 2, 3, 4, 5]];
        let result = count_safe_reports(lists);
        let want = 1;
        assert_eq!(want, result);
    }

    #[test]
    fn count_safe_reports_test_expect_2() {
        let lists = vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]];
        let result = count_safe_reports(lists);
        let want = 2;
        assert_eq!(want, result);
    }

    #[test]
    fn count_safe_reports_test_expect_0() {
        let lists = vec![vec![1, 2, 3, 4, 9], vec![9, 4, 3, 2, 1]];
        let result = count_safe_reports(lists);
        let want = 0;
        assert_eq!(want, result);
    }

    #[test]
    fn report_direction_safe_test_expect_safe() {
        let list = vec![1, 2, 3, 4, 5];
        let result = report_direction_safe(&list);
        let want = true;
        assert_eq!(want, result);
    }

    #[test]
    fn report_direction_safe_test_expect_unsafe() {
        let list = vec![1, 2, 3, 4, 3];
        let result = report_direction_safe(&list);
        let want = false;
        assert_eq!(want, result);
    }

    #[test]
    fn count_safe_reports_test_example() {
        let lists = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let result = count_safe_reports(lists);
        let want = 2;
        assert_eq!(want, result);
    }
}
