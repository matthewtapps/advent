use std::fs;

fn main() {
    println!("Hello, world!");
}

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).unwrap();
    let mut lists: Vec<Vec<i32>> = Vec::new();

    for (_, line) in contents.split("\n").enumerate() {
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
}
