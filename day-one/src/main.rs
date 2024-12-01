use std::fs;

fn main() {
    let (list_one, list_two) = parse_file("input.txt");
    println!("{}", get_list_distance(list_one.clone(), list_two.clone()));
}

fn get_list_distance(list_one: Vec<i32>, list_two: Vec<i32>) -> i32 {
    let mut sorted_list_one = list_one.clone();
    sorted_list_one.sort();
    let mut sorted_list_two = list_two.clone();
    sorted_list_two.sort();

    let longest_list: Vec<i32> = if sorted_list_one > sorted_list_two {
        sorted_list_one.clone()
    } else {
        sorted_list_two.clone()
    };

    let other_list = if sorted_list_one > sorted_list_two {
        sorted_list_two.clone()
    } else {
        sorted_list_one.clone()
    };

    let mut total_distance = 0;

    for (index, list_one_element) in longest_list.iter().enumerate() {
        let list_two_element = other_list[index];

        let distance = list_one_element - list_two_element;

        total_distance += distance.abs();
    }

    return total_distance;
}


fn parse_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_path).unwrap();
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
    for (index, item) in contents.split_whitespace().enumerate() {
        let integer: i32 = item.parse().unwrap();
        if index % 2 == 1 {
            list_one.push(integer)
        } else {
            list_two.push(integer)
        }
    }
    return (list_one, list_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_distance_test_expect_3() {
        let list_one = vec![1, 2, 3];
        let list_two = vec![4, 3, 2];
        let want = 3;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn list_distance_test_expect_2() {
        let list_one = vec![1, 2, 4];
        let list_two = vec![4, 3, 2];
        let want = 2;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn list_distance_test_expect_0() {
        let list_one = vec![1, 2, 3];
        let list_two = vec![1, 2, 3];
        let want = 0;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn list_distance_test_differing_lengths() {
        let list_one = vec![2, 3];
        let list_two = vec![1, 2, 3];
        let want = 2;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn list_distance_test_differing_lengths_reversed() {
        let list_one = vec![1, 2, 3];
        let list_two = vec![2, 3];
        let want = 2;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn list_distance_test_example() {
        let list_one = vec![3, 4, 2, 1, 3, 3];
        let list_two = vec![4, 3, 5, 3, 9, 3];
        let want = 11;
        let result = get_list_distance(list_one, list_two);
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_one() {
        let result = parse_file("./mocks/mock1.txt");
        let want = (vec![2, 4, 6], vec![1, 3, 5]);
        assert_eq!(want, result);
    }

    #[test]
    fn parse_file_test_two() {
        let result = parse_file("./mocks/mock2.txt");
        let want = (vec![2, 4, 6], vec![1, 3, 5]);
        assert_eq!(want, result);
    }
}
