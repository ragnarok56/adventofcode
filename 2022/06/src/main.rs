use std::{fs};

fn part(n: usize) -> usize {
    let lines: Vec<char> = fs::read_to_string("in").unwrap().chars().collect();

    lines
        .windows(n)
        .enumerate()
        .filter(|x| {
            let mut marker = x.1.to_vec();
            marker.sort();
            marker.dedup();
            marker.len() == n
        })
        .map(|x| x.0 + n)
        .next()
        .unwrap()
}

fn main() {
    let result = part(4);
    println!("{:?}", result);
    let result = part(14);
    println!("{:?}", result);
}