use std::collections::HashMap;
use std::{fs::File};
use std::io::{BufReader, BufRead};

#[derive(Copy, Clone, Debug)]
struct Tile {
    t: char,
    r: usize,
    c: usize,
    p: char
}

impl Tile {
    fn is_empty(&self) -> bool {
        self.t == ' '
    }
    fn is_open(&self) -> bool {
        self.t == '.'
    }
    fn is_blocked(&self) -> bool {
        self.t == '#'
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Heading {
    N = 3,
    E = 0,
    S = 1,
    W = 2
}

impl Heading {
    fn move_amount(&self) -> i32 {
        match self {
            Heading::N => -1,
            Heading::E => 1,
            Heading::S => 1,
            Heading::W => -1
        }
    }
}

#[derive(Clone, Debug)]
struct Player {
    heading: Heading,
    r: usize,
    c: usize
}

fn update_heading(heading: Heading, command: String) -> Heading {
    match command.as_str() {
        "R" => match heading {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N
        },
        "L" => match heading {
            Heading::N => Heading::W,
            Heading::E => Heading::N,
            Heading::S => Heading::E,
            Heading::W => Heading::S
        },
        _ => heading
    }
}

fn get_heading_char(heading: &Heading) -> char {
    match heading {
        Heading::N => '^',
        Heading::E => '>',
        Heading::S => 'v',
        Heading::W => '<'
    }
}

fn display_board(board: &Vec<Vec<Tile>>, player: &Player) {
    let max = 10;
    for (r, x) in board.iter().enumerate() {
        if r >= max {
            return
        }
        for (c, t) in x.iter().enumerate() {
            if player.r == r && player.c == c {
                print!("P");
                continue
            }
            let tile_display = match t.p {
                ' ' => t.t,
                _ => t.p
            };
            print!("{}", tile_display);
        }
        println!();
    }
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part() {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let binding: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let max_c = binding.iter().take_while(|x| x.len() > 0).map(|x| x.len()).max().unwrap();
    let mut binding = binding.iter();
    let board_input = binding.by_ref().take_while(|x| x.len() > 0);

    let mut board: Vec<Vec<Tile>> = board_input
        .enumerate()
        .map(|(r, l)| {
            let extra = " ".repeat(max_c - l.len());
            let full_line = format!("{}{}", l, extra);
            full_line
                .chars()
                .enumerate()
                .map(|(c, t)| Tile{ t: t, r: r, c: c, p: ' ' })
                .collect()
        })
        .collect();

    let binding = binding.next().unwrap();
    let commands = binding
        .split_inclusive(['R','L'])
        .map(|c| {
            let split = c.split_at(c.len() - 1);
            let split =  vec![split.0.to_string(), split.1.to_string()];
            split
                .iter()
                .filter(|x| x.len() > 0)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .flat_map(|x| x);

    let transposed_board = &mut transpose(&board);

    let row_start_end: HashMap<usize, (Tile, Tile)> = board
        .iter()
        .by_ref()
        .enumerate()
        .map(|(r, x)| {
            let start = x.iter().filter(|x| !x.is_empty()).next().unwrap().clone();
            let end = x.iter().rev().filter(|x| !x.is_empty()).next().unwrap().clone();
            (r, (start, end))
        })
        .collect();
    let col_start_end: HashMap<usize, (&Tile, &Tile)> = transposed_board
        .iter()
        .enumerate()
        .map(|(c, x)| {
            let start = x.iter().filter(|x| !x.is_empty()).next().unwrap();
            let end = x.iter().rev().filter(|x| !x.is_empty()).next().unwrap();
            (c, (start, end))
        })
        .collect();
    

    let start_position = board.get(0).unwrap().iter().filter(|c| c.is_open()).next().unwrap();
    let mut player = Player{ heading: Heading::E, r: start_position.r, c: start_position.c };

    for c in commands {
        let moves = c.parse::<usize>();
        if moves.is_ok() {
            let moves = moves.ok().unwrap();
            println!("{}", moves);
            if player.heading == Heading::N || player.heading == Heading::S {
                let col = transposed_board.get(player.c).unwrap();
                println!("Moving {:?} {:?} in col {:?}", moves, player.heading, player.c);
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.r as i32 + move_amount) as usize;
                    let start_end = col_start_end.get(&player.c).unwrap();
                    // println!("start_end: {:?}", start_end);
                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };
                    let tile_pot = col.get(dest).unwrap_or(wrap_tile);
                    // println!("Next Tile: {:?}", tile_pot);

                    if tile_pot.is_blocked() {
                        println!("player blocked at row: {:?}, staying at {:?}", dest, player.r);
                        break;
                    // } else if tile_pot.is_empty() {
                    //     // check tile on other side is blocked or open
                        
                    //     if dest > player.r {
                    //         let tile_wrap = col.iter().filter(|x| !x.is_empty()).next().unwrap();
                    //         if tile_wrap.is_open() {
                    //             player.r = tile_wrap.r;
                    //         }
                    //     } else if dest < player.c {
                    //         let tile_wrap = col.iter().rev().filter(|x| !x.is_empty()).next().unwrap();
                    //         if tile_wrap.is_open() {
                    //             player.r = tile_wrap.r;
                    //         }
                    //     }
                    } else {
                        player.r = tile_pot.r;
                    }
                    // println!("player {:?}", player);
                    println!("player {:?}, {:?}", player, board.get_mut(player.c).unwrap().get_mut(player.r).unwrap());
                    board.get_mut(player.r).unwrap().get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                    println!("player {:?}, {:?}", player, board.get_mut(player.c).unwrap().get_mut(player.r).unwrap());
                }
                
            } else {
                let row = board.get_mut(player.r).unwrap();
                println!("Moving {:?} {:?} in row {:?}", moves, player.heading, player.r);
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.c as i32 + move_amount) as usize;
                    let start_end = row_start_end.get(&player.r).unwrap();
                    // println!("start_end: {:?}", start_end);
                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };
                    let tile_pot = row.get(dest).unwrap_or(&wrap_tile);
                    if tile_pot.is_blocked() {
                        println!("player blocked at col: {:?}, staying at {:?}", dest, player.c);
                        break;
                    // } else if tile_pot.is_empty() {
                    //     // check tile on other side is blocked or open
                    //     if dest > player.c {
                    //         let tile_wrap = row.iter().filter(|x| !x.is_empty()).next().unwrap();
                    //         if tile_wrap.is_open() {
                    //             player.c = tile_wrap.c;
                    //         }
                    //     } else if dest < player.c {
                    //         let tile_wrap = row.iter().rev().filter(|x| !x.is_empty()).next().unwrap();
                    //         if tile_wrap.is_open() {
                    //             player.c = tile_wrap.c;
                    //         }
                    //     }
                    } else {
                        player.c = dest;
                    }
                    // println!("player {:?}", player);
                    println!("player {:?}, {}, {:?}", player, row.len(), row.get_mut(player.c).unwrap());
                    // board.get_mut(player.r).unwrap().get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                    row.get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                }
            }
        } else {
            player.heading = update_heading(player.heading, c);
        }
        display_board(&board, &player);
    }
    // println!("{:?}", player);
    println!("{}", 1000 * (player.r + 1) + 4 * (player.c + 1) + (player.heading as usize))

}

fn main() {
    part();
}
