extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    // it is really hard to just parse some args...
    let matches = App::new("adventofcode/2021/08")
        .arg(Arg::with_name("filename")
            .short("f")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("sum_outputs")
            .long("sum-outputs")
            .takes_value(false)
            .required(false))
        .get_matches();
    let filename = matches.value_of("filename").unwrap();

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn sort_string(s: String) -> String {
        let mut str_chars: Vec<char> = s.chars().collect();
        str_chars.sort();
        return String::from_iter(str_chars);
    }

    if let Ok(lines) = read_lines(filename) {
        let result = lines.map(|l| {
            let input = l.unwrap();
            let mut inputs_iter = input.split("|");
            let digits: Vec<&str> = inputs_iter.next().unwrap_or("").split_whitespace().collect();
            let outputs: Vec<&str> = inputs_iter.next().unwrap_or("").split_whitespace().collect();

            let count_known_digits = || {
                return outputs
                    .iter()
                    .map(|x| x.chars().count())
                    .map(|x|
                        match x {
                            2 | 3 | 4 | 7 => x,
                            _ => 0
                        }
                    )
                    .count();
            };

            let sum_outputs = || {
                let digit_count_map = digits
                    .iter()
                    .flat_map(|x| x.split("").map(str::to_owned).collect::<Vec<_>>())
                    .fold(HashMap::new(), |mut acc, x| {
                        *acc.entry(x).or_insert(0) += 1;
                        acc
                    });

                let digit_identifier_map = digits
                    .iter()
                    .fold(HashMap::new(), |mut acc, x| {
                        *acc.entry(sort_string(x.to_string())).or_insert(0) += x.chars().map(|c| {
                            return digit_count_map.get(&c.to_string()).unwrap_or(&0)
                        }).sum::<i32>();
                        acc
                    });

                return outputs
                    .iter()
                    .map(|x| {
                        let sorted_digits = sort_string(x.to_string());
                        return match digit_identifier_map.get(&sorted_digits).unwrap() {
                            17 => 1,
                            25 => 7,
                            30 => 4,
                            34 => 2,
                            37 => 5,
                            39 => 3,
                            41 => 6,
                            42 => 0,
                            45 => 9,
                            49 => 8,
                            _ => -1
                        };
                    })
                    .filter(|x| x != &-1)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse::<usize>().unwrap_or(0);
            };

            match matches.is_present("sum_outputs") {
                true => sum_outputs(),
                false => count_known_digits(),
            }
        });

        println!("{:?}", result.sum::<usize>());
    }
}