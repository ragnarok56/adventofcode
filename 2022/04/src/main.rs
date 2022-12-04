use std::{fs};

fn part1() -> u32 {
    let lines: Vec<String> = fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    lines.iter().map(|line| {
            let mut sections = line.split(',');
            let mut a = sections.next().unwrap().split('-');
            let mut b = sections.next().unwrap().split('-');
            let a1: i32 = a.next().unwrap().parse().unwrap();
            let a2: i32 = a.next().unwrap().parse().unwrap();
            let b1: i32 = b.next().unwrap().parse().unwrap();
            let b2: i32 = b.next().unwrap().parse().unwrap();
            match (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2) {
                true => 1,
                false => 0
            }
        })
        .sum()
}


fn main() {
    let result = part1();
    println!("{:?}", result);
}