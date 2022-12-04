use std::{fs};

struct CampRange {
    begin: i32,
    end: i32
}

fn check_overlap(a: CampRange, b: CampRange) -> bool {
    a.begin <= b.end && a.end >= b.begin
}

fn check_encompass(a: CampRange, b: CampRange) -> bool {
    (a.begin >= b.begin && a.end <= b.end) || (b.begin >= a.begin && b.end <= a.end)
}

fn part(check: fn(CampRange, CampRange) -> bool) -> u32 {
    let lines: Vec<String> = fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    lines
        .iter()
        .map(|line| {
            let mut sections = line.split(',');
            let mut a = sections.next().unwrap().split('-');
            let mut b = sections.next().unwrap().split('-');
            let camp_a = CampRange {
                begin: a.next().unwrap().parse().unwrap(),
                end: a.next().unwrap().parse().unwrap()
            };
            let camp_b = CampRange {
                begin: b.next().unwrap().parse().unwrap(),
                end: b.next().unwrap().parse().unwrap()
            };
            match check(camp_a, camp_b) {
                true => 1,
                false => 0
            }
        })
        .sum()
}

fn main() {
    let result = part(check_encompass);
    println!("{:?}", result);
    let result = part(check_overlap);
    println!("{:?}", result);
}