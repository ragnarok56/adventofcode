use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let re_p1 = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let p1: i32 = input
        .lines()
        .flat_map(|x| {            
            re_p1.captures_iter(x).map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        })
        .sum();
    println!("{:?}", p1);
    
    let re_p2 = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))|(do\(\))|(don\'t\(\))").unwrap();
    let init = (0, true);
    let p2: i32 = input
        .lines()
        .fold(init, |acc, x| {          
            re_p2
                .captures_iter(x)
                .fold(acc, |(mut sum, mut on), c| {
                    if c.get(4).is_some() { on = true; }
                    else if c.get(5).is_some() { on = false; } 
                    else if on { sum += c[2].parse::<i32>().unwrap() * c[3].parse::<i32>().unwrap() }
                    (sum, on)
                })
        }).0;
    println!("{:?}", p2);
}
