extern crate clap;
use clap::{Arg, App};
use std::fs;

fn main() {
    // it is really hard to just parse some args...
    let matches = App::new("adventofcode/2021/07")
        .arg(Arg::with_name("filename")
            .short("f")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("true_crab_cost")
            .long("true-crab-cost")
            .takes_value(false)
            .required(false))
        .get_matches();
    let filename = matches.value_of("filename").unwrap();
    let true_crab_cost = matches.is_present("true_crab_cost");

    let crab_positions: Vec<i32> = fs::read_to_string(filename)
        .expect("failed to read file")
        .split(",")
        .map(|s| s.trim().to_string().parse().unwrap())
        .collect();

    let min = crab_positions.iter().min().unwrap();
    let max = crab_positions.iter().max().unwrap();

    let cheapest_fuel: Vec<i32> = (*min..*max).map(|x|
        crab_positions.iter().map(|c|
            match true_crab_cost {
                false => (x - c).abs(),
                true => (0..(x - c).abs()).sum(),
            }
        ).sum()
    ).collect();
    println!("{:?}", cheapest_fuel.iter().min().unwrap());
}