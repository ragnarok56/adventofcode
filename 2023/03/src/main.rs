use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Part {
    value: u32,
    x: usize,
    start: usize,
    end: usize,
}

fn parse_matrix() -> Vec<Vec<char>> {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap());

    lines
        .map(|x| {
            x.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x);
                acc
            })
        })
        .collect()
}

fn get_pos(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> &char {
    matrix.get(x).unwrap().get(y).unwrap()
}

fn boundary_check(matrix: &Vec<Vec<char>>, x: usize, y: usize, func: fn(&char) -> bool) -> Vec<(usize, usize)> {
    let mut matching_positions: Vec<(usize, usize)> = Vec::new();
    for rx in 0..3 {
        for ry in 0..3 {
            if rx + x == 0 {
                continue
            }
            if ry + y == 0 {
                continue
            }
            let cx = rx + x - 1;
            let cy = ry + y - 1;
            if cx > matrix.len() - 1 {
                continue;
            }
            if cy > matrix.get(x).unwrap().len() - 1 {
                continue;
            }
            let val = get_pos(&matrix, cx, cy);
            if func(val) {
                matching_positions.push((cx, cy));
            }
        }
    }
    matching_positions
}

fn next_to_symbol(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    boundary_check(matrix, x, y, |c| !c.is_ascii_digit() && *c != '.').len() > 0
}

fn gear_positions(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    boundary_check(matrix, x, y, |c| c.is_ascii_digit())
}

fn get_parts(matrix: &Vec<Vec<char>>) -> Vec<Part> {
    let mut part_start_y: Option<usize> = None;
    let mut part_num: Option<u32> = None;
    let mut parts: Vec<Part> = Vec::new();
    for (x, _) in matrix.iter().enumerate() {
        for (y, _) in matrix.get(x).unwrap().iter().enumerate() {
            let val = get_pos(&matrix, x, y);
            if val.is_ascii_digit() {
                let d = val.to_digit(10).unwrap();
                // found potential part number, create new or add to current part number
                if part_num.is_none() {
                    part_start_y = Some(y);
                    part_num = Some(d);
                } else {
                    part_num = Some(part_num.unwrap() * 10 + d);
                }
            } 
            if y == matrix.get(x).unwrap().len() - 1 || !val.is_ascii_digit() {
                // reached end of potential part number
                if part_start_y.is_some() {
                    // need to check if its a valid part
                    let range_start = part_start_y.unwrap();
                    let range_end = y;
                    let mut is_valid = false;
                    for i in range_start..range_end {
                        if next_to_symbol(&matrix, x, i) {
                            is_valid = true;
                            break; 
                        }
                    }
                    if is_valid {
                        parts.push(Part{ value: part_num.unwrap(), x, start: part_start_y.unwrap(), end: y})
                    }
                    part_start_y = None;
                    part_num = None;
                    // println!()
                }
            }
        }
        // reset each row
        part_start_y = None;
        part_num = None;
    }
    parts
}


fn main() {

    let matrix = parse_matrix();
    let parts = get_parts(&matrix);

    println!("p1: {:?}",parts.iter().map(|x| x.value).sum::<u32>());

    let mut ratios = 0;
    let mut gears: HashSet<u32> = HashSet::new();
    for (i, _) in matrix.iter().enumerate() {
        for (j, _) in matrix.get(i).unwrap().iter().enumerate() {
            if *get_pos(&matrix, i, j) == '*' {
                let positions = gear_positions(&matrix, i, j);
                for pos in positions {
                    for part in parts.iter() {
                        if part.x == pos.0 && part.start <= pos.1 && part.end >= pos.1 {
                            gears.insert(part.value);
                            break;
                        }
                    }
                }
                if gears.len() == 2 {
                    ratios += gears.iter().fold(1, |mut acc, x| {
                        acc = acc * x;
                        acc
                    })
                }
                gears.clear();
            }
        }
    }
    println!("p2: {:?}", ratios)
}
