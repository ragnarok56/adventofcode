use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_input() -> Vec<Vec<u32>> {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap());

    lines
        .map(|x| {
            x.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x.to_digit(10).unwrap());
                acc
            })
        })
        .collect()
}

fn part1() {
    let matrix = parse_input();

    let mut visible_count = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            // edges
            if x == 0 || x == matrix.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_count += 1;
                continue
            }

            // inside, check visibility
            // check row
            let (row_left, row_right) = row.split_at(y);
            if row_left.iter().all(|r| r < value) {
                visible_count += 1;
                continue
            }
            if row_right.iter().skip(1).all(|r| r < value) {
                visible_count += 1;
                continue
            }

            // check cols
            let col: Vec<u32> = matrix.iter().map(|r| *r.get(y).unwrap()).collect();
            let (col_top, col_bottom) = col.split_at(x);
            if col_top.iter().all(|r| r < value) {
                visible_count += 1;
                continue
            }
            if col_bottom.iter().skip(1).all(|r| r < value) {
                visible_count += 1;
                continue
            }
        }
    }
    println!("{:?}", visible_count);
}

fn check_view(height: u32, iter: impl Iterator<Item = u32>) -> u32 {
    let mut score = 0;
    for x in iter {
        if x < height {
            score += 1;
        }
        if x >= height {
            score += 1;
            break;
        }
    }
    return score
}

fn part2() {
    let matrix = parse_input();

    let mut max_scenic_score = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, value) in row.iter().map(|x| *x).enumerate() {
            // edges
            if x == 0 || x == matrix.len() - 1 || y == 0 || y == row.len() - 1 {
                continue
            }
            // check row
            let (row_left, row_right) = row.split_at(y);
            let left_scenic_score = check_view(value, row_left.iter().rev().map(|x| *x));
            let right_scenic_score = check_view(value, row_right.iter().skip(1).map(|x| *x));

            // check cols
            let col: Vec<u32> = matrix.iter().map(|r| *r.get(y).unwrap()).collect();
            let (col_top, col_bottom) = col.split_at(x);
            let top_scenic_score = check_view(value, col_top.iter().rev().map(|x| *x));
            let bottom_scenic_score = check_view(value, col_bottom.iter().skip(1).map(|x| *x));

            let scenic_score = left_scenic_score * right_scenic_score * top_scenic_score * bottom_scenic_score;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }
    println!("{:?}", max_scenic_score);
}

fn main() {
    part1();
    part2();
}