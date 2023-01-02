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

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
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

fn _display_board(board: &Vec<Vec<Tile>>, player: &Player) {
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

fn get_tile(board: &Vec<Vec<Tile>>, row: usize, col: usize) -> Option<&Tile> {
    board.get(row).unwrap().get(col)
}

fn get_new_position(fs: usize, row: usize, col: usize, cur_heading: Heading, next_heading: Heading) -> (usize, usize) {
    let new_col = col % fs;
    let new_row = row % fs;

    let (r, c) = match (cur_heading, next_heading) {
        (Heading::N, Heading::E) => (new_col, 0),
        (Heading::W, Heading::S) => (0, new_row),
        (Heading::S, Heading::W) => (new_col, (fs - 1)),
        (Heading::E, Heading::N) => ((fs - 1), new_row),
        (Heading::E, Heading::W) => ((fs - 1) - new_row, (fs - 1)),
        (Heading::W, Heading::E) => ((fs - 1) - new_row, 0),
        (Heading::S, Heading::S) => (0, new_col),
        (Heading::N, Heading::N) => ((fs - 1), new_col),
        _ => panic!("woops")
    };
    // println!("{:?} to {:?}, [{},{}]({},{}) to ({},{})", cur_heading, next_heading, row, col, new_row, new_col, r, c);
    (r, c)
}

fn part1() -> () {
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
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let binding: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let max_c = binding.iter().take_while(|x| x.len() > 0).map(|x| x.len()).max().unwrap();
    let mut binding = binding.iter();
    let board_input = binding.by_ref().take_while(|x| x.len() > 0);

    // size of a cube face
    let fs = 50;
    // regions that make up cubes, used for looking up which faces connect later
    let regions = vec![(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)];

    let face_ranges: Vec<((usize,usize), (usize,usize))> = regions.iter()
        .map(|x| {
            ((x.0 * fs, x.1 * fs), (((x.0 + 1) * fs) - 1, ((x.1 + 1) * fs) - 1))
        })
        .collect();
    let region_pos: HashMap<usize,(usize, usize)> = regions.into_iter().enumerate().collect();
    let region_map: HashMap<(usize, &Heading),(usize, Heading)> = HashMap::from_iter([
        ((0, &Heading::N), (5, Heading::E)),
        ((0, &Heading::W), (3, Heading::E)),
        ((1, &Heading::N), (5, Heading::N)),
        ((1, &Heading::E), (4, Heading::W)),
        ((1, &Heading::S), (2, Heading::W)),
        ((2, &Heading::W), (3, Heading::S)),
        ((2, &Heading::E), (1, Heading::N)),
        ((3, &Heading::N), (2, Heading::E)),
        ((3, &Heading::W), (0, Heading::E)),
        ((4, &Heading::E), (1, Heading::W)),
        ((4, &Heading::S), (5, Heading::W)),
        ((5, &Heading::W), (0, Heading::S)),
        ((5, &Heading::E), (4, Heading::N)),
        ((5, &Heading::S), (1, Heading::S))
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
            for _ in 0..moves {
                let cur_player_heading = player.heading.clone();
                let move_amount = player.heading.move_amount();
                let tile_pot: &Tile;
                if cur_player_heading == Heading::N || cur_player_heading == Heading::S {
                    // let col = transposed_board.get(player.c).unwrap();
                    let dest = (player.r as i32 + move_amount) as usize;
                    let start_end = col_start_end.get(&player.c).unwrap();

                    if dest < start_end.0.r || dest > start_end.1.r {
                        let cur_tile = get_tile(&transposed_board, player.c, player.r).unwrap();
                        let next_region = region_map.get(&(cur_tile.face.unwrap(), &cur_player_heading)).unwrap();
                        player.heading = next_region.1;
                        let (mut new_row, mut new_col) = get_new_position(fs, player.r, player.c, cur_player_heading, next_region.1);
                        let mult = region_pos.get(&next_region.0).unwrap();
                        new_row = (fs * mult.0) + new_row;
                        new_col = (fs * mult.1) + new_col;
                        tile_pot = get_tile(&transposed_board, new_col, new_row).unwrap();
                    } else {
                        tile_pot = get_tile(&transposed_board, player.c, dest).unwrap();
                    }
                } else {
                    let move_amount = player.heading.move_amount();
                    let dest = (player.c as i32 + move_amount) as usize;
                    let start_end = row_start_end.get(&player.r).unwrap();

                    if dest < start_end.0.c || dest > start_end.1.c {
                        let cur_tile = get_tile(&board, player.r, player.c).unwrap();
                        let next_region = region_map.get(&(cur_tile.face.unwrap(), &cur_player_heading)).unwrap();
                        player.heading = next_region.1;
                        let (mut new_row, mut new_col) = get_new_position(fs, player.r, player.c, cur_player_heading, next_region.1);

                        let mult = region_pos.get(&next_region.0).unwrap();
                        new_row = (fs * mult.0) + new_row;
                        new_col = (fs * mult.1) + new_col;
                        tile_pot = get_tile(&board, new_row, new_col).unwrap();
                    } else {
                        tile_pot = get_tile(&board, player.r, dest).unwrap();
                    }
                }
                if tile_pot.is_blocked() {
                    player.heading = cur_player_heading;
                    break;
                } else {
                    player.r = tile_pot.r;
                    player.c = tile_pot.c;
                }
                board.get_mut(player.r).unwrap().get_mut(player.c).unwrap().p = get_heading_char(&player.heading);
            }
        } else {
            player.heading = update_heading(player.heading, c);
        }
    }
    // _display_board(&board, &player);
    println!("{}", 1000 * (player.r + 1) + 4 * (player.c + 1) + (player.heading as usize));
}

fn main() {
    part1();
    part2();
}
