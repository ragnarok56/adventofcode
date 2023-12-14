use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct SpringStatus {
    damaged: Vec<u32>
}

impl SpringStatus {
    fn is_valid(&self, series: String) -> bool {
        let split = series.split('.').filter(|x| !x.is_empty()).collect::<Vec<_>>();

        if self.damaged.len() != split.len() {
            return false
        }

        return split.iter().enumerate().all(|(i, s)| (s.len() as u32) == self.damaged[i])
    }
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();

    let mut p1 = 0;
    for l in lines {
        let mut split = l.split_whitespace();
        let series = split.next().unwrap().to_string();
        let damaged = split.next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let status = SpringStatus{ damaged };

        // just throw # or . at each ?
        let iter = series
            .chars()
            .map(|x| if x == '?' { "#.".chars().collect_vec() } else { vec![x] } );

        p1 += iter
            .multi_cartesian_product()
            .filter(|x| status.is_valid(x.into_iter().collect::<String>()))
            .count();
    }

    println!("p1: {:?}", p1);
}