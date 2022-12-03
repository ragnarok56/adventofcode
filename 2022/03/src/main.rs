use std::{fs, collections::HashSet};

fn part1() -> u32 {
    fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|line: &str| {
            let compartments = line.split_at(line.len() / 2);
            let left_compartment = compartments.0.chars().fold(HashSet::new(), |mut acc, x| {
                acc.insert(x);
                acc
            });
            let right_compartment = compartments.1.chars().fold(HashSet::new(), |mut acc, x| {
                acc.insert(x);
                acc
            });
            left_compartment
                .intersection(&right_compartment)
                .map(|x| {
                    match x {
                        'a'..='z' => *x as u32 - 96,
                        'A'..='Z' => *x as u32 - 64 + 26,
                        _ => 0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

// fn part2() -> i32{
//     return 0
// }

fn main() {
    let result = part1();
    println!("{:?}", result);
    // let result = part2();
    // println!("{:?}", result);
}