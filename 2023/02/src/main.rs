use std::fs;
use std::collections::HashMap;
// use std::cmp::Ordering;
// use clap::Parser;

fn main() {
    println!("Hello, world!");

    let mut valid_game: HashMap<&str, i32> = HashMap::new();
    valid_game.insert("red", 12);
    valid_game.insert("green", 13);
    valid_game.insert("blue", 14);

    let filename = "in_test";
    let p1_sum = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let draws = x.split(":")
                .skip(1)
                .take(1)
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .split(";")
                .collect::<Vec<&str>>();

            let valid_game = draws
                .iter()
                .map(|d| {
                    let cubes = d.split(",")
                        .map(|c| {
                            let mut citer = c.split_whitespace();
                            let var_name = (citer.next().unwrap().parse::<i32>().unwrap(), citer.next().unwrap());
                            var_name
                        })
                        .collect::<Vec<_>>();

                    cubes.iter().all(|c| valid_game.get(c.1).unwrap() >= &c.0)
                })
                .all(|d| d);
            (i, valid_game)
        })
        .fold(0,|mut acc, (i, x)| {
            if x {
                acc += i;
            }
            acc
        });

    println!("{:?}", p1_sum);

}
