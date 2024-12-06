use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let input = parse_file("input.txt");
    let (rules, lists) = parse_input(&input);
    let result = get_valid_lists_score(&rules, &lists);
    println!("{}", result)
}

fn get_relevant_rules_for_lists(rules: &Vec<(i32, i32)>, lists: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    return rules
        .clone()
        .iter()
        .map(|rule| *rule)
        .filter(|rule| {
            let (left, right) = *rule;

            for list in lists {
                if list.contains(&left) || list.contains(&right) {
                    return true;
                }
            }
            return false;
        })
        .collect::<Vec<_>>();
}

fn get_relevant_rules_for_list(rules: &Vec<(i32, i32)>, list: &Vec<i32>) -> Vec<(i32, i32)> {
    return rules
        .clone()
        .iter()
        .map(|rule| *rule)
        .filter(|rule| {
            let (left, right) = *rule;

            return list.contains(&left) && list.contains(&right);
        })
        .collect::<Vec<_>>();
}

fn list_is_valid(rules: &Vec<(i32, i32)>, list: &Vec<i32>) -> bool {
    for (index, element) in list.iter().enumerate() {
        for rule in rules {
            let (left, right) = rule;
            if right == element {
                let parent_index = list.iter().position(|&elem| elem == *left).unwrap();

                if parent_index >= index {
                    return false;
                }
            }
        }
    }
    return true;
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

fn parse_input(parsed_file: &Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = vec![];
    let mut lists: Vec<Vec<i32>> = vec![];
    for input_line in parsed_file {
        if input_line.contains('|') {
            let numbers: Vec<i32> = input_line
                .split('|')
                .map(|number| number.parse().unwrap())
                .collect();
            rules.push((numbers[0], numbers[1]))
        }
        if input_line.contains(',') {
            let numbers: Vec<i32> = input_line
                .split(',')
                .map(|number| number.parse().unwrap())
                .collect();
            lists.push(numbers)
        }
    }

    return (rules, lists);
}

fn get_middle_elements(lists: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut elements = Vec::new();
    for list in lists {
        let middle_index = list.len() / 2;
        let middle_element = list[middle_index];
        elements.push(middle_element);
    }
    return elements;
}

fn get_valid_lists_score(rules: &Vec<(i32, i32)>, lists: &Vec<Vec<i32>>) -> i32 {
    let relevant_rules = get_relevant_rules_for_lists(&rules, &lists);
    let mut valid_lists = Vec::new();
    for list in lists {
        let relevant_rules_for_list = get_relevant_rules_for_list(&relevant_rules, &list);
        if list_is_valid(&relevant_rules_for_list, &list) {
            valid_lists.push(list.clone());
        }
    }

    let middle_elements = get_middle_elements(&valid_lists);

    let mut total = 0;

    for element in middle_elements {
        total += element;
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_relevant_rules_for_lists_test() {
        let rules = vec![(1, 3), (5, 4)];
        let lists = vec![vec![1, 2, 3, 4, 5]];
        let result = get_relevant_rules_for_lists(&rules, &lists);
        let want = vec![(1, 3), (5, 4)];
        assert_eq!(want, result);
    }

    #[test]
    fn get_rules_where_element_is_child_test() {
        let rules = vec![(1, 3), (5, 4), (9, 23)];
        let list = vec![1, 2, 3, 4, 5];
        let result = get_relevant_rules_for_list(&rules, &list);
        let want = vec![(1, 3), (5, 4)];
        assert_eq!(want, result);
    }

    #[test]
    fn list_is_valid_test_expect_false() {
        let rules = vec![(1, 3), (5, 4)];
        let list = vec![1, 2, 3, 4, 5];
        let result = list_is_valid(&rules, &list);
        let want = false;
        assert_eq!(want, result);
    }

    #[test]
    fn list_is_valid_test_expect_true() {
        let rules = vec![(1, 3), (4, 5)];
        let list = vec![1, 2, 3, 4, 5];
        let result = list_is_valid(&rules, &list);
        let want = true;
        assert_eq!(want, result);
    }

    #[test]
    fn parse_input_test() {
        let input = parse_file("./mocks/mock1.txt");
        let result = parse_input(&input);
        let want = (
            vec![(21, 3), (3, 8)],
            vec![vec![3, 5, 2, 1], vec![1, 2, 3, 4]],
        );
        assert_eq!(want, result);
    }

    #[test]
    fn get_middle_elements_test() {
        let input = vec![vec![1, 2, 3, 4, 5]];
        let result = get_middle_elements(&input);
        let want = vec![3];
        assert_eq!(want, result);
    }

    #[test]
    fn part_one_acceptance_test() {
        let input = parse_file("./mocks/example.txt");
        let (rules, lists) = parse_input(&input);
        let result = get_valid_lists_score(&rules, &lists);
        let want = 143;
        assert_eq!(want, result);
    }
}
