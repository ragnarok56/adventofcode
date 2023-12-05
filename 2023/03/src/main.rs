use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct EnginePart {
    value: u32,
    pos: Vec<Coord>
}

#[derive(Debug)]
struct Symbol {
    value: char,
    pos: Coord
}

fn parse_matrix() -> Vec<Vec<char>> {
    let file = File::open("in_test").expect("file doesnt exist");

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

// fn parse_input() -> (Vec<EnginePart>, Vec<Symbol>) {
//     let file = File::open("in_test").expect("file doesnt exist");

//     let reader = BufReader::new(file);

//     let mut parts: Vec<EnginePart> = Vec::new();
//     let mut symbols: Vec<Symbol> = Vec::new();

//     let lines = reader
//         .lines()
//         .enumerate()
//         .for_each(|line| {
//             let x = line.0;
//             let mut cur_part: Option<&EnginePart> = None;
//             line.1.unwrap().chars().enumerate().for_each(|c| {
//                 let y = c.0;
//                 let pos = Coord{ x: x, y: y };
//                 match c.1 {
//                     '.' => {
//                         if cur_part.is_some() {
//                             cur_part = None;
//                         }
//                     },
//                     _ => {
//                         match c.1.is_numeric() {
//                             true => {
//                                 let digit = c.1.to_digit(10).unwrap();
//                                 if cur_part.is_none() {
//                                     let part = EnginePart{ value: digit, pos: vec![pos]};
//                                     parts.push(part);
//                                     cur_part = Some(&part);
//                                 } else {
//                                     cur_part.unwrap().value = cur_part.as_mut().unwrap().value * 10 + digit;
//                                     cur_part.as_mut().unwrap().pos.push(pos);
//                                 }
//                             }
//                             false => {
//                                 symbols.push(Symbol{ value: c.1, pos: pos});
//                             }

//                         }
//                     }
//                 }
//             });
//             cur_part = None
//         });
//         (parts, symbols)
// }

fn get_pos(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> &char {
    matrix.get(x).unwrap().get(y).unwrap()
}

fn next_to_symbol(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut is_next_to: bool = false;
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
            // println!("checking: {:?}, {:?}", cx, cy);
            match matrix.get(cx).unwrap().get(cy).unwrap() {
                '*' | '#' | '$' | '+' => is_next_to = true,
                _ => ()
            }
        }
    }
    is_next_to
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for (x, i) in matrix.iter().enumerate() {
        print!("{:?}|", x);
        for (y, j) in i.iter().enumerate() {
            print!("{j}");
        }
        println!();
    }
}

fn get_part(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    for i in matrix.get(x).unwrap().len() {
        matrix.get(x).
    }
}


fn main() {

    let matrix = parse_matrix();
    let visited: HashMap<(usize,usize), bool> = HashMap::new();
    let parts: Vec<i32> = Vec::new();
    print_matrix(&matrix);

    for (x, i) in matrix.iter().enumerate() {
        for (y, j) in i.iter().enumerate() {
    for (x, i) in matrix.iter().enumerate() {
        for (y, j) in i.iter().enumerate() {
            if get_pos(&matrix, x, y).is_alphanumeric() {
                let is_next_to_symbol = next_to_symbol(&matrix, x, y);
                if is_next_to_symbol {

                    println!("{:?}, {:?}", x, y);

                }
            }
        }
        // println!("{:?}", next_to_symbol(&matrix, 0, 1));
    }
    println!("{:?}",matrix);
}
