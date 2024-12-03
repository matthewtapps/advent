use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    unimplemented!()
}

// thanks stackoverflow
fn parse_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let contents = File::open(file_path).expect("no such file");
    let buf = BufReader::new(contents);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .collect()
}

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
}
