use std::collections::HashMap;
use std::{fs::File};
use std::io::{BufReader, BufRead};

fn display(coords: &HashMap<(i32,i32), String>) {

    let x_range = (coords.keys().map(|x| x.0).min().unwrap(), coords.keys().map(|x| x.0).max().unwrap() + 1);
    let y_range = (coords.keys().map(|x| x.1).min().unwrap(), coords.keys().map(|x| x.1).max().unwrap() + 1);

    println!("{:?}", coords);

    println!("{:?}, {:?}", x_range, y_range);

    let mut show_start = true;
    for y in 0..y_range.1 {
        if show_start {
            for x in x_range.0..x_range.1 {
                if x == 500 {
                    print!("!");
                } else {
                    print!(" ");
                }
            }
            show_start = false;
            println!();
            println!();
        }
        for x in x_range.0..x_range.1 {
            if coords.contains_key(&(x, y)) {
                print!("{}", coords.get(&(x, y)).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}


fn part(part_1: bool) {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let lines = reader.lines()
        .map(|x| {
            x.unwrap()
                .split(" -> ")
                .map(|s| {
                    let mut split = s.split(",");
                    let x:i32 = split.next().unwrap().parse().unwrap();
                    let y:i32 = split.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect::<Vec<(i32,i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let mut coords: HashMap<(i32, i32), String> = HashMap::new();

    for l in lines {
        for pair in l.windows(2) {
            let s = pair[0];
            let e = pair[1];
            if s.0 == e.0 {
                let start = e.1.min(s.1);
                let stop = e.1.max(s.1);
                for x in start..=stop {
                    coords.insert((s.0, x), "#".to_string());
                }
            } else {
                let start = e.0.min(s.0);
                let stop = e.0.max(s.0);
                for x in start..=stop {
                    coords.insert((x, s.1), "#".to_string());
                }
            }
        }
    }

    let floor = coords.keys().map(|x| x.1).max().unwrap() + 1;

    let mut stop_condition = false;
    let mut num_sands = 0;
    let moves = [(0, 1), (-1, 1), (1, 1)];
    while !stop_condition {
        num_sands += 1;
        let mut sand = (500, 0);
        let mut rest = false;
        while !rest {
            let mut c = 3;
            for m in moves {
                let next_pos = (sand.0 + m.0, sand.1 + m.1);
                if !coords.contains_key(&next_pos) {
                    sand = next_pos;
                    break
                }
                c -= 1;
            }
            if c == 0 || (!part_1 && sand.1 == floor) {
                rest = true;
            }

            if part_1 && sand.1 >= floor {
                stop_condition = true;
                break;
            }
            if !part_1 && sand == (500, 0) && rest {
                stop_condition = true;
                break;
            }
        }
        coords.insert(sand, "o".to_string());
    }
    display(&coords);
    let result = num_sands + if part_1 { -1 } else { 0 };
    println!("{:?}", result);
}

fn main() {
    part(true);
    part(false);
}
