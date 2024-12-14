use std::fs;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum D {
    N, S, E, W
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn is_valid(&self, board: &Vec<Vec<char>>) -> bool {
        self.x < 0 || self.y < 0 || self.x + 1 > board.len() || self.y + 1 > board.first().unwrap().len()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Guard {
    pos: Pos,
    d: D
}

fn get_board_cell(board: &Vec<Vec<char>>, x: usize, y: usize) -> &char {
    board.get(y).unwrap().get(x).unwrap()
}

impl Guard {
    fn try_move_guard(&mut self, board: &Vec<Vec<char>>, w: usize, h: usize)-> bool {
        match self.d {
            D::E => {
                if self.pos.x == w - 1 {
                    false
                } else {
                    if *get_board_cell(board, self.pos.x + 1, self.pos.y) == '#' {
                        self.d = D::S;
                    } else {
                        self.pos.x += 1;
                    }
                    true
                }
            },
            D::W => {
                if self.pos.x == 0 {
                    false
                } else {
                    if *get_board_cell(board, self.pos.x - 1, self.pos.y) == '#' {
                        self.d = D::N;
                    } else {
                        self.pos.x -= 1;
                    }
                    true
                }
            },
            D::N => {
                if self.pos.y == 0 {
                    false
                } else {
                    if *get_board_cell(board, self.pos.x, self.pos.y - 1) == '#' {
                        self.d = D::E;
                    } else {
                        self.pos.y -= 1;
                    }
                    true
                }
            },
            D::S => {
                if self.pos.y == h - 1 {
                    false
                } else {
                    if *get_board_cell(board, self.pos.x, self.pos.y + 1) == '#' {
                        self.d = D::W;
                    } else {
                        self.pos.y += 1;
                    }
                    true
                }
            }
        }
    }
}


fn load_input(path: &str) -> (Guard, Vec<Vec<char>>) {
    let mut guard: Option<Guard> = None;
    let board = fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, x)| {
            x.chars().enumerate().fold(Vec::new(), |mut acc, (j, x)| {
                let dir: Option<D> = match x {
                    '^' => Some(D::N),
                    'v' => Some(D::S),
                    '>' => Some(D::E),
                    '<' => Some(D::W),
                    _ => None
                };
                if dir.is_some() {
                    guard = Some(Guard{pos: Pos{x: j, y: i}, d: dir.unwrap()});
                    acc.push('.');
                } else {
                    acc.push(x);
                }
                acc
            })
        })
        .collect();

    (guard.unwrap(), board)
}

fn print_board(board: &Vec<Vec<char>>) {
    for r in board.iter() {
        for c in r.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let (mut guard, board) = load_input("in_test");
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut visited_and_dir: HashSet<(Pos, D)> = HashSet::new();
    visited.insert(guard.pos);
    visited_and_dir.insert((guard.pos, guard.d));
    print_board(&board);
    println!("{:?}", guard);


    let w = board.len();
    let h = board.first().unwrap().len();
    let mut loop_options = 0;
    while guard.try_move_guard(&board, w, h) {
        // println!("{:?}", guard);
        visited.insert(guard.pos);
        visited_and_dir.insert((guard.pos, guard.d));

        // this only checks if guard passes by an previously passed spot
        // but needs to account for guard passing by a _line_ of previously passed spots
        // that they could turn and walk into, thus causing a loop.  need to track what those
        // spots are and how to look them up based on guards current position and direction.
        let found = match guard.d {
            D::W => visited_and_dir.contains(&(Pos{x: guard.pos.x, y: guard.pos.y - 1}, D::N)),
            D::E => visited_and_dir.contains(&(Pos{x: guard.pos.x, y: guard.pos.y + 1}, D::S)),
            D::S => visited_and_dir.contains(&(Pos{x: guard.pos.x - 1, y: guard.pos.y}, D::W)),
            D::N => visited_and_dir.contains(&(Pos{x: guard.pos.x + 1, y: guard.pos.y}, D::E))
        };
        if found {
            loop_options += 1;
        }

    }

    println!("{:?}", visited.len());
    println!("{:?}", loop_options);
}