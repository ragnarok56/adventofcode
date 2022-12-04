use std::{fs, collections::HashSet};

fn priority_score(x: &char) -> u32 {
    // determine score based on character
    // a-z = 1 - 26, A-Z = 26-52
    match x {
        'a'..='z' => *x as u32 - 96,
        'A'..='Z' => *x as u32 - 64 + 26,
        _ => 0
    }
}
trait HashSetChars {
    fn chars_to_hashset(&self) -> HashSet<char>;
}

impl HashSetChars for str {
    // get unique set of characters in a string
    fn chars_to_hashset(&self) -> HashSet<char> {
        self.chars().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        })
    }
}

fn part1() -> u32 {
    fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|line: &str| {
            let compartments = line.split_at(line.len() / 2);
            let left_compartment = compartments.0.chars_to_hashset();
            let right_compartment = compartments.1.chars_to_hashset();
            left_compartment
                .intersection(&right_compartment)
                .map(priority_score)
                .sum::<u32>()
        })
        .sum()
}

fn part2() -> u32{
    let lines: Vec<String> = fs::read_to_string("in")
        .unwrap()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    lines
        .chunks(3)
        .map(|x| {
            let sacks = x
                .iter()
                .map(|s| s.chars_to_hashset())
                .collect::<Vec<HashSet<char>>>();

            sacks
                .iter()
                .skip(1)
                .fold(sacks[0].clone(), |acc, s| {
                    acc.intersection(s).cloned().collect()
                })
                .iter()
                .map(priority_score)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let result = part1();
    println!("{:?}", result);
    let result = part2();
    println!("{:?}", result);
}