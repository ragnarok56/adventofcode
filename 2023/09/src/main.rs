use std::fs;

fn main() {
    let input = fs::read_to_string("in_test").unwrap();
    let lines = input.lines();

    let reports = lines
        .map(|x| x.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", reports);
}