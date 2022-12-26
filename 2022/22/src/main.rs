use std::collections::HashMap;
use std::{fs::File};
use std::io::{BufReader, BufRead};

#[derive(Copy, Clone, Debug)]
struct Tile {
    t: char,
    r: usize,
    c: usize,
    p: char,
    face: Option<usize>
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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
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

    fn toggle_orientation(&self, other: Heading) -> bool {
        if *self == other {
            return false
        } else if (*self == Heading::N || *self == Heading::S) && (other == Heading::E || other == Heading::W) {
            return true
        } else if (*self == Heading::E || *self == Heading::W) && (other == Heading::N || other == Heading::S) {
            return true
        }
        return false
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

fn cube_sides(board: &Vec<Vec<Tile>>) {
    let side_length = 4;
    let x = board.len() / side_length;
    let y = board.get(0).unwrap().len() / side_length;
    let mut sides: Vec<Vec<u32>> = Vec::new();
    for i in 0..x {
        let mut r: Vec<u32> = Vec::new();
        for j in 0..y {
            let is_side = !board.get(i * side_length).unwrap().get(j * side_length).unwrap().is_empty();
            r.push(is_side as u32);
        }
        sides.push(r);
    }
    for s in sides {
        for c in s {
            print!("{}", c);
        }
        println!();
    }
}

fn part1() -> () {
    let file = File::open("in_test").expect("file doesnt exist");

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
                .map(|(c, t)| Tile{ t: t, r: r, c: c, p: ' ', face: Some(0) })
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
            if player.heading == Heading::N || player.heading == Heading::S {
                let col = transposed_board.get(player.c).unwrap();
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.r as i32 + move_amount) as usize;
                    let start_end = col_start_end.get(&player.c).unwrap();

                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };

                    let tile_pot;
                    if dest < start_end.0.r || dest > start_end.1.r {
                        tile_pot = wrap_tile;
                    } else {
                        tile_pot = col.get(dest).unwrap_or(wrap_tile);
                    }

                    if tile_pot.is_blocked() {
                        break;
                    } else {
                        player.r = tile_pot.r;
                    }
                    board.get_mut(player.r).unwrap().get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                }
                
            } else {
                let row = board.get_mut(player.r).unwrap();
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.c as i32 + move_amount) as usize;
                    let start_end = row_start_end.get(&player.r).unwrap();

                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };

                    let tile_pot;
                    if dest < start_end.0.c || dest > start_end.1.c {
                        tile_pot = &wrap_tile;
                    } else {
                        tile_pot = row.get(dest).unwrap_or(&wrap_tile);
                    }

                    if tile_pot.is_blocked() {
                        break;
                    } else {
                        player.c = tile_pot.c;
                    }
                    row.get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                }
            }
        } else {
            player.heading = update_heading(player.heading, c);
        }
    }
    println!("{}", 1000 * (player.r + 1) + 4 * (player.c + 1) + (player.heading as usize))
}


fn part2() -> () {
    let file = File::open("in_test").expect("file doesnt exist");

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let binding: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let max_c = binding.iter().take_while(|x| x.len() > 0).map(|x| x.len()).max().unwrap();
    let mut binding = binding.iter();
    let board_input = binding.by_ref().take_while(|x| x.len() > 0);
    
    let fs = 4;
    let face_ranges = vec![
        ((0, fs * 2), (fs - 1, 2 * fs * 2 - 1)),
        ((fs, 0), (fs * 2 - 1, fs - 1)),
        ((fs, fs), (fs * 2 - 1, fs * 2 - 1)),
        ((fs, fs * 2), (fs * 2 - 1, fs * 3 - 1)),
        ((fs * 2, fs * 2), (fs * 3 - 1, fs * 3 - 1)),
        ((fs * 2, fs * 3), (fs * 3 - 1, fs * 4 - 1))
    ];

    let example_region_map: HashMap<(usize, &Heading),(usize, Heading)> = HashMap::from_iter([
        ((0, &Heading::N), (1, Heading::S)),
        ((0, &Heading::E), (5, Heading::W)),
        ((0, &Heading::W), (2, Heading::S)),
        ((1, &Heading::N), (0, Heading::S)),
        ((1, &Heading::S), (4, Heading::N)),
        ((1, &Heading::W), (5, Heading::N)),
        ((2, &Heading::N), (1, Heading::E)),
        ((2, &Heading::S), (4, Heading::E)),
        ((3, &Heading::E), (5, Heading::S)),
        ((4, &Heading::S), (1, Heading::N)),
        ((4, &Heading::W), (2, Heading::N)),
        ((5, &Heading::N), (3, Heading::W)),
        ((5, &Heading::E), (0, Heading::W)),
        ((5, &Heading::S), (1, Heading::E))
    ]);

    let mut board: Vec<Vec<Tile>> = board_input
        .enumerate()
        .map(|(r, l)| {
            let extra = " ".repeat(max_c - l.len());
            let full_line = format!("{}{}", l, extra);
            full_line
                .chars()
                .enumerate()
                .map(|(c, t)| {
                    let face = face_ranges
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| {
                            // println!("x: {:?}, r: {}, c: {}", x, r, c);
                            x.0.0 <= r && r <= x.1.0 && x.0.1 <= c && c <= x.1.1
                        })
                        .map(|x| x.0)
                        .next();
                    Tile{ t: t, r: r, c: c, p: ' ', face: face }
                })
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
            if player.heading == Heading::N || player.heading == Heading::S {
                let col = transposed_board.get(player.c).unwrap();
                println!("Moving {:?} {:?} in col {:?}", moves, player.heading, player.c);
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.r as i32 + move_amount) as usize;
                    let start_end = col_start_end.get(&player.c).unwrap();

                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };

                    let tile_pot;
                    if dest < start_end.0.r || dest > start_end.1.r {
                        tile_pot = wrap_tile;
                        println!("move would wrap, {} outside bounds {}-{}, new tile_pot is {:?}", dest, start_end.0.r, start_end.1.r, tile_pot);
                    } else {
                        tile_pot = col.get(dest).unwrap_or(wrap_tile);
                        println!("move is on board, new tile is {:?}", tile_pot);
                    }

                    if tile_pot.is_blocked() {
                        println!("player blocked at row: {:?}, staying at {:?}", dest, player.r);
                        break;
                    } else {
                        player.r = tile_pot.r;
                    }
                    board.get_mut(player.r).unwrap().get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                }
                
            } else {
                let row = board.get_mut(player.r).unwrap();
                println!("Moving {:?} {:?} in row {:?}", moves, player.heading, player.r);
                let move_amount = player.heading.move_amount();
                for _ in 0..moves {
                    let dest = (player.c as i32 + move_amount) as usize;
                    let start_end = row_start_end.get(&player.r).unwrap();

                    let wrap_tile;
                    if move_amount < 0 { 
                        wrap_tile = start_end.1
                    } else { 
                        wrap_tile = start_end.0 
                    };

                    let tile_pot;
                    if dest < start_end.0.c || dest > start_end.1.c {
                        tile_pot = &wrap_tile;
                        println!("current player tile: {:?}", row.get(player.c).unwrap());
                        let next_region = example_region_map.get(&(row.get(player.c).unwrap().face.unwrap(), &player.heading));
                        println!("move would wrap, {} outside bounds {}-{}, new tile_pot is {:?}, next_region: {:?}", dest, start_end.0.c, start_end.1.c, tile_pot, next_region);
                    } else {
                        tile_pot = row.get(dest).unwrap_or(&wrap_tile);
                        println!("move is on board, new tile is {:?}", tile_pot);
                    }

                    if tile_pot.is_blocked() {
                        println!("player blocked at col: {:?}, staying at {:?}", dest, player.c);
                        break;
                    } else {
                        player.c = tile_pot.c;
                    }
                    row.get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
                }
            }
        } else {
            player.heading = update_heading(player.heading, c);
        }
    }
    display_board(&board, &player);
    println!("{}", 1000 * (player.r + 1) + 4 * (player.c + 1) + (player.heading as usize));
}

fn main() {
    // part1();
    part2();
}
