use std::fs;

#[derive(Debug)]
struct Board {
    rows: Vec<String>,
    cols: Vec<String>
}

impl Board {
    fn add_row(&mut self, row: String) {
        self.rows.push(row);
    }
    fn add_col(&mut self, col: String) {
        self.cols.push(col);
    }
}

fn get_reflected(v: &Vec<String>) -> usize {
    // unga bunga
    let mut stack: Vec<String> = Vec::new();
    let mut popped_lines: Vec<String> = Vec::new();
    let mut reflection_idx = 0;
    for (i, r) in v.iter().enumerate() {
        if stack.len() == 0 {
            stack.push(r.to_string());
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
                    popped_lines.insert(0, r.to_string());
                    // nothing left on stack, we're at end
                    if stack.len() == 0 {
                        break;
                    }
                } else if reflection_idx == 0 {
                    // no reflection found yet, increment stack
                    stack.push(r.to_string());
                } else if stack.len() > 0 {
                    // maybe ran into a match before the end, add back popped lines
                    for p in popped_lines.iter() {
                        stack.push(p.to_string());
                    }
                    stack.push(r.to_string());
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
            println!("==================================================");
            let score = (100 * get_reflected(&x.rows)) + get_reflected(&x.cols);
            // if score == 0 {
                println!("[{:?}] {:?}", i, x);
                println!("score: {}", score);
            // }
            score
        })
        .sum();
    println!("p1: {}", p1);
}