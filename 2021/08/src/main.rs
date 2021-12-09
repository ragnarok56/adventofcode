extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // it is really hard to just parse some args...
    let matches = App::new("adventofcode/2021/08")
        .arg(Arg::with_name("filename")
            .short("f")
            .takes_value(true)
            .required(true))
        .get_matches();
    let filename = matches.value_of("filename").unwrap();

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    if let Ok(lines) = read_lines(filename) {
        let known_digits = lines.map(|l| {
            let input = l.unwrap();
            let mut inputs_iter = input.split("|");
            let outputs: Vec<&str> = inputs_iter.nth(1).unwrap_or("").split_whitespace().collect();

            return outputs
                .iter()
                .map(|x| x.chars().count())
                .filter(|x| x == &2 || x == &3 || x == &4 || x == &7)
                .count();
        });

        println!("{:?}", known_digits.sum::<usize>());
    }
}