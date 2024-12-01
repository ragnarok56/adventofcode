use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("in").unwrap();

    let split: Vec<Vec<&str>> = input.lines().map(|x| {
        return x.split("   ").collect::<Vec<&str>>()
    }).collect();

    let mut left: Vec<i32> = split.iter().map(|x| { x[0].parse::<i32>().unwrap() }).collect();
    let mut right: Vec<i32> = split.iter().map(|x| { x[1].parse::<i32>().unwrap() }).collect();
    left.sort();
    right.sort();

    let p1: i32 = left.iter().zip(right.clone()).map(|(l, r)| { (l - r).abs() }).sum();

    println!("{:?}", p1);

    let right_map = right.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let p2: i32 = left.iter().map(|x| { right_map.get(&x).or(Some(&0)).unwrap() * x }).sum();
    println!("{:?}", p2);

}
