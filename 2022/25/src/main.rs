use std::{fs::File};
use std::io::{BufReader, BufRead};

fn to_dec(num: String) -> String {
    let mut dec_value = 0;
    for (p, n) in num.chars().rev().enumerate() {
        let n_val: i64 = match n {
            '-' => -1,
            '=' => -2,
            _ => n.to_string().parse::<i64>().unwrap()
        };
        let place_dec_value = 5i64.pow(p as u32);
        dec_value = dec_value + (n_val * place_dec_value)
    }
    dec_value.to_string()
}

fn to_snafu(num: String) -> String {
    let mut snafu_value: Vec<String> = Vec::new();
    let mut con_num = num.parse::<i64>().unwrap();
    while con_num != 0 {
        let mut rem = con_num % 5;
        if rem > 2 {
            con_num = con_num + rem;
            rem = rem - 5;
        }
        con_num = con_num / 5;
        let place_snafu_value = match rem {
            -2 => "=".to_string(),
            -1 => "-".to_string(),
            _ => rem.to_string()
        };
        snafu_value.push(place_snafu_value.to_string());
    }
    snafu_value.into_iter().rev().collect::<Vec<String>>().join("")
}

fn part1() {
    let file = File::open("in").expect("file doesnt exist");

    let sum: i64 = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| to_dec(x).parse::<i64>().unwrap())
        .sum();

    println!("{}", sum);
    println!("{}", to_snafu(sum.to_string()));
}

fn main() {
    part1();
}