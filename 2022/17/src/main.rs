use std::collections::HashMap;
use std::mem::{replace};
use std::{fs::File};
use std::io::{BufReader, BufRead};

#[derive(Copy, Clone)]
enum PieceType {
    HLINE,
    CROSS,
    L,
    VLINE,
    SQUARE
}

trait PieceIntersects {
    fn intersects(&self, tower: Vec<[i32;7]>) -> bool;
}

fn part1(num: u32, lookback: u32) {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let mut piece_iter = [PieceType::HLINE, PieceType::CROSS,PieceType::L,PieceType::VLINE,PieceType::SQUARE].iter().enumerate().cycle();
    let binding = reader.lines().map(|x| x.unwrap().chars().collect::<Vec<char>>()).next().unwrap();
    let mut wind_iter = binding.iter().enumerate().cycle();

    let mut tower: Vec<[i32;7]> = Vec::new();
    let empty = [0,0,0,0,0,0,0];
    tower.push([1,1,1,1,1,1,1]);
    for _ in 0..3 {
        tower.push(empty);
    }
    let mut max_height = 0;
    let mut cycle_map: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut heights: Vec<i32> = Vec::new();
    let mut height_before_loop = 0;
    let mut blocks_landed_before_loop = 0;

    /*
    0 index starts at bottom.  "down" movement subtracts one

    4 |.......|
    3 |.......|
    2 |.......|
    1 |.......|
    0 +-------+
    */

    for n in 0..num {
        let mut rest = false;
        let next_piece = piece_iter.next().unwrap();
        let piece = match next_piece.1 {
            PieceType::HLINE  => [[1,1,1,1].to_vec()].to_vec(),
            PieceType::CROSS  => [[0,1,0].to_vec(),[1,1,1].to_vec(),[0,1,0].to_vec()].to_vec(),
            PieceType::L      => [[0,0,1].to_vec(),[0,0,1].to_vec(),[1,1,1].to_vec()].to_vec(),
            PieceType::VLINE  => [[1].to_vec(),[1].to_vec(),[1].to_vec(),[1].to_vec()].to_vec(),
            PieceType::SQUARE => [[1,1].to_vec(),[1,1].to_vec()].to_vec()
        };

        /*
        4 |..####.|
        3 |.......|
        2 |.......|
        1 |.......|
        0 +-------+
        max_height = 0
        piece_height = 1
        starting_height = max_height + 3 + piece_height = 0 + 3 + 1 = 4
        piece_bottom_row = starting_height - piece_height + (piece_height - 3) = 4 - 1 + (1 - 3) = 3
        7 |...@...|
        6 |..@@@..|
        5 |...@...|
        4 |.......|
        3 |.......|
        2 |.......|
        1 |..####.|
        0 +-------+
        max_height = 1
        piece_height = 3
        starting_height = max_height + 3 + piece_height = 1 + 3 + 3 = 7
        piece_bottom_row = starting_height - piece_height + (piece_height - 3) = 7 - 3 + (3 - 3) = 4
        10|....@..|
        9 |....@..|
        8 |..@@@..|
        7 |.......|
        6 |.......|
        5 |.......|
        4 |...#...|
        3 |..###..|
        2 |...#...|
        1 |..####.|
        0 +-------+
        max_height = 4
        piece_height = 3
        starting_height = max_height + 3 + piece_height = 4 + 3 + 3 = 10
        piece_bottom_row = starting_height - piece_height + (piece_height - 3) = 10 - 3 + (3-3) = 7
        22|..@....|
        21|..@....|
        20|..@....|
        19|..@....|
        18|.......|
        17|.......|
        16|.......|
        15|.....#.|
        14|.....#.|
        13|..####.|
        12|.###...|
        11|..#....|
        10|.####..|
        9 |....##.|
        8 |....##.|
        7 |....#..|
        6 |..#.#..|
        5 |..#.#..|
        4 |#####..|
        3 |..###..|
        2 |...#...|
        1 |..####.|
        0 +-------+
        max_height = 15
        piece_height = 4
        starting_height = max_height + 3 + piece_height = 15 + 3 + 4 = 22
        piece_bottom_row = starting_height - piece_height + (piece_height - 3) = 22 - 4 + (4 - 3) = 19
        */
        let piece_height = piece.len() as i32;
        let starting_height = max_height + 3 + piece_height;
        let mut piece_bottom_row = starting_height - piece_height;
        if piece_height > 3 {
            piece_bottom_row = piece_height - 3 + piece_bottom_row;
        }
    
        let mut piece_tl: (i32, i32) = (starting_height, 2);

        // println!("New Piece! max height: {}, piece height: {}, starting_height: {}, piece_bottom_row: {}", max_height, piece_height, starting_height, piece_bottom_row);

        while !rest {
            let wind = wind_iter.next().unwrap();
            // println!("{:?}", wind);
            let mut piece_tl_pot = match wind.1 {
                '<' => (piece_tl.0, piece_tl.1 - 1),
                '>' => (piece_tl.0, piece_tl.1 + 1),
                _ => piece_tl
            };
            let mut can_move = true;
            for (r, x) in piece.iter().enumerate() {
                for (c, y) in x.iter().enumerate() {
                    if *y != 0 {
                        // println!("Checking {} ({},{}) -> [{},{}]", wind, r, c, piece_tl_pot.0 - (r as i32), (c as i32) + piece_tl_pot.1);
                        if ((c as i32) + piece_tl_pot.1 < 0) || ((c as i32) + piece_tl_pot.1 >= 7) {
                            can_move = false;
                        // } else if piece_tl_pot.0 < piece_bottom_row && *tower.get((piece_tl_pot.0 as usize) - r).unwrap().get(c + (piece_tl_pot.1 as usize)).unwrap() != 0 {
                        } else if piece_tl_pot.0 < piece_bottom_row {
                            let tower_row = tower.get((piece_tl_pot.0 as usize) - r);                                
                            if tower_row.is_some() {
                                let tower_col = tower_row.unwrap().get(((c as i32) + piece_tl_pot.1) as usize);
                                if tower_col.is_some() {
                                    if *tower_col.unwrap() != 0 {
                                        // println!("cant move: intersection");
                                        can_move = false;
                                    }
                                } else {
                                    // println!("cant move: col is None");
                                    can_move = false;
                                }
                            } else {
                                // println!("row is None");
                            }
                        } else {
                            // println!("skipping checks, not past buffer zone");
                        }
                    }
                }
            }
            if can_move {
                // println!("Wind moved piece! [{}]", wind);
                piece_tl = piece_tl_pot;
            }

            can_move = true;
            piece_tl_pot = (piece_tl.0 - 1, piece_tl.1);
            for (r, x) in piece.iter().enumerate() {
                for (c, y) in x.iter().enumerate() {
                    if *y != 0 {
                        // println!("Checking D ({},{}) -> [{},{}]", r, c, piece_tl_pot.0 - (r as i32), c + (piece_tl_pot.1 as usize));
                        if piece_tl_pot.0 < piece_bottom_row {
                            let tower_row = tower.get((piece_tl_pot.0 as usize) - r);                                
                            if tower_row.is_some() {
                                let tower_col = tower_row.unwrap().get(c + (piece_tl_pot.1 as usize));
                                if tower_col.is_some() {
                                    if *tower_col.unwrap() != 0 {
                                        // println!("cant move: intersection");
                                        can_move = false;
                                    }
                                } else {
                                    // println!("cant move: col is None");
                                    can_move = false;
                                }
                            } else {
                                // println!("row is None");
                            }
                        }
                    }
                }
            }
            if can_move {
                // println!("Moved down one!");
                piece_tl = piece_tl_pot;
            } else {
                rest = true;
                // println!("Piece Rested at {:?}", piece_tl);
                for (r, x) in piece.iter().enumerate() {
                    for (c, y) in x.iter().enumerate() {
                        if *y != 0 {
                            // println!("Placing ({},{}) -> [{},{}]", r, c, piece_tl.0 - (r as i32), (c as i32) + piece_tl.1);
                            if piece_tl.0 - (r as i32) > max_height {
                                max_height = (r as i32) + piece_tl.0;
                            }
                            // println!("{} > {}?", (piece_tl.0 as usize) - r, tower.len());
                            if (piece_tl.0 as usize) - r >= tower.len() {
                                // println!("adding to tower");
                                let diff = (piece_tl.0 as usize) - r - tower.len() + 1;
                                for _ in 0..diff {
                                    tower.push(empty);
                                }
                            }
                            // println!("{} > {}?", (piece_tl.0 as usize) - r, tower.len());
                            let _ = replace(&mut tower[(piece_tl.0 as usize) - r][c + (piece_tl.1 as usize)], 1);
                        }
                    }
                }
                if lookback > 0 {
                    let rows = tower.iter().rev()
                        .take(lookback as usize)
                        .map(|x| x.iter().map(|c| c.to_string()).collect::<Vec<String>>())
                        .reduce(|mut acc, x| {
                            for s in x.iter() {
                                acc.push(s.to_string());
                            }
                            acc
                        })
                        .unwrap()
                        .join("");
                    let entry = format!("{:?}-{:?}-{:?}", rows, wind.0, next_piece.0);
                    let content = cycle_map.entry(entry).or_insert(Vec::new());
                    content.push((max_height, (n as i32)));
                    if content.len() == 2 && height_before_loop == 0 {
                        height_before_loop = max_height;
                        blocks_landed_before_loop = n;
                    }
                    // *cycle_map.entry(entry).or_default() = (max_height, );
                }
                heights.push(max_height);
            }
        }
    }
    // for (i, r) in tower.iter().rev().enumerate() {
    //     if i == tower.len() - 1 { print!("+") } else { print!("|") }
    //     for c in r {
    //         if *c == 0 {
    //             print!(".");
    //         } else {
    //             if i == tower.len() - 1 { print!("-") } else { print!("#") }
    //         }
    //     }
    //     if i == tower.len() - 1 { print!("+") } else { print!("|") }
    //     println!()
    // }
    let mut cycle_rocks: i64 = 0;
    let mut cycle_height_diff: i64 = 0;
    for x in cycle_map.iter() {
        if x.1.len() > 1 {
            println!("{:?}: {:?}", x.0, x.1);
            let mut cycle_iter = x.1.iter();
            let first = cycle_iter.next().unwrap();
            let second = cycle_iter.next().unwrap();
            cycle_rocks = (second.1 - first.1) as i64;
            cycle_height_diff = (second.0 - first.0) as i64;
            println!("blocks before loop: {:?}", blocks_landed_before_loop);
            println!("height_before_loop: {:?}", height_before_loop);
            let cycles = (1000000000000i64 - blocks_landed_before_loop as i64) as i64 / cycle_rocks;
            println!("cycles: {:?}", cycles);
            println!("remaining: {:?}", 1000000000000i64 - (cycles * cycle_rocks + blocks_landed_before_loop as i64));
            println!("rocks: {:?}, height diff: {:?}", cycle_rocks, cycle_height_diff);
            // println!("heights: {:?}", &heights[cycle_rocks as usize..(cycle_rocks as usize)*2 + 1]);
            break
        }
    }
    println!("max height: {}", max_height);
    println!("rocks: {}", 1000000000000 / cycle_rocks);
    println!("max height for elephants to be impressed: {}", (1000000000000 / cycle_rocks * cycle_height_diff));
    // 1560932944583
    // 1560932944615  <-- answer
    // 32 off, wtf
    
}

fn main() {
    part1(2022, 50);
}
