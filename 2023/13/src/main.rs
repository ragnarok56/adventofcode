use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};


#[derive(Debug)]
struct Board {
    rows: Vec<u64>,
    cols: Vec<u64>
}

impl Board {
    fn add_row(&mut self, row: String) {
        self.rows.push(calculate_hash(&row));
    }
    fn add_col(&mut self, col: String) {
        self.cols.push(calculate_hash(&col));
    }
}

fn get_reflected(v: &Vec<u64>) -> usize {
    // unga bunga
    let mut stack: Vec<u64> = Vec::new();
    let mut popped_lines: Vec<u64> = Vec::new();
    let mut reflection_idx = 0;
    for (i, r) in v.iter().enumerate() {
        if stack.len() == 0 {
            stack.push(*r);
        } else {
            let last = stack.last();
            if last.is_some() {
                // println!("{} | {}", last.unwrap(), r);
                if last.unwrap() == r {
                    // maybe a reflection, set index
                    if reflection_idx == 0 {
                        reflection_idx = i;
                    }
                    // track 
                    popped_lines.push(stack.pop().unwrap());
                    // copy to front to keep ordering ugh
                    popped_lines.insert(0, *r);
                    // nothing left on stack, we're at end
                    if stack.len() == 0 {
                        break;
                    }
                } else if reflection_idx == 0 {
                    // no reflection found yet, increment stack
                    stack.push(*r);
                } else if stack.len() > 0 {
                    // maybe ran into a match before the end, add back popped lines
                    for p in popped_lines.iter() {
                        stack.push(*p);
                    }
                    stack.push(*r);
                    popped_lines.clear();
                    reflection_idx = 0;
                } else {
                    // not a reflection
                    reflection_idx = 0;
                    break;
                }
            }
        }
        // println!("stack: {:?}, popped: {:?}", stack, popped_lines);
    }
    // println!("{}", reflection_idx);
    reflection_idx
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();

    let mut board_list: Vec<Vec<&str>> = Vec::new();
    let mut board: Vec<&str> = Vec::new();
    for l in lines {
        if l.is_empty() {
            board_list.push(board);
            board = Vec::new();
        } else {
            board.push(l);
        }
    }
    board_list.push(board);

    let boards = board_list.iter()
        .map(|x| {
            let mut board = Board { rows: Vec::new(), cols: Vec::new() };
            let col_range = 0..x[0].len();
            for r in x.iter() {
                board.add_row(r.to_string());
            }
            for c in col_range {
                let col_vals: String = x.iter().map(|x| x.chars().collect::<Vec<_>>()[c]).collect();
                // println!("{:?}", col_vals);
                board.add_col(col_vals);
            }
            board
        })
        .collect::<Vec<_>>();

    let p1: usize = boards.iter().enumerate()
        .map(|(i, x)| {
            let score = (100 * get_reflected(&x.rows)) + get_reflected(&x.cols);
            if score == 0 {
                println!("[{:?}] {:?}", i, x);
                println!("score: {}", score);
            }
            score
        })
        .sum();
    println!("p1: {}", p1);
}