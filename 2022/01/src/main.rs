extern crate clap;
use std::fs;
use std::cmp::Ordering;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   filename: String,
}

#[derive(PartialEq, Debug, Clone)]
struct Elf {
	food: Vec<i32>
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;

    let input = fs::read_to_string(filename).unwrap();

    let mut elves = Vec::<Elf>::new();
    let elf = Elf {
        food: Vec::<i32>::new()
    };
    elves.push(elf);
    let mut cur_elf = 0;
    // yup, parsing strings is like, super hard
    for line in input.lines() {
        if line.chars().count() == 0 {
            let elf = Elf {
                food: Vec::<i32>::new()
            };
            elves.push(elf);
            cur_elf = cur_elf + 1
        } else {
            elves[cur_elf].food.push(line.parse::<i32>().unwrap());
        }
    }

    let largest = elves.iter().fold(0, |mut acc, x| {
        let sum = x.food.iter().sum();
        if sum > acc {
            acc = sum;
        }
        acc
    });

    println!("most calories: {:?}", largest);

    elves.sort_by(|a, b| {
        let a_sum: i32 = a.food.iter().sum();
        let b_sum: i32 = b.food.iter().sum();
        if a_sum > b_sum {
            Ordering::Less
        } else if a_sum == b_sum {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let top_3_elves = elves[0..3].iter().fold(0, |acc, x| {
        // let sum: i32 = x.food.iter().sum();
        acc + x.food.iter().sum::<i32>()
    });
    println!("top 3 sum: {:?}", top_3_elves)
}