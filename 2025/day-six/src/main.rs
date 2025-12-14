use std::path::Path;

const INPUT: &str = "input.txt";

fn parse_file(file_path: impl AsRef<Path>) -> Vec<i64> {
    let contents = std::fs::read_to_string(file_path).expect("no such file");

    let mut lines: Vec<&str> = contents.trim().split("\n").collect();

    // extract the last line as the operations
    let operations_line = lines.pop().expect("failed to pop last line");

    // split operations line into individual &str operations for later
    let operations: Vec<_> = operations_line.split_whitespace().collect();

    // parse each line into it's set of values
    let rows: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i64>().expect("couldn't parse value"))
                .collect()
        })
        .collect();

    // retrieve the number of columns from the first row
    let num_columns = rows.first().map(|r| r.len()).unwrap_or(0);

    // for each column, extract its values from each row individually
    // and apply the corresponding operation for that column
    let results: Vec<i64> = (0..num_columns)
        .map(|col_idx| {
            let operation = operations[col_idx];
            let column: Vec<i64> = rows.iter().map(|row| row[col_idx]).collect();
            match operation {
                "+" => column.iter().sum(),
                "*" => column.iter().product(),
                _ => panic!("unsupported operation: {}", operation),
            }
        })
        .collect();

    // return a vec of the result of each column
    results
}

fn main() {
    let results = parse_file(INPUT);

    // print the sum of the column results
    println!("{:?}", results.iter().sum::<i64>());
}
