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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Guard {
    pos: Pos,
    d: D
}

impl Guard {
    fn next_pos(&self, w: usize, h: usize) -> Option<Pos> {
        match self.d {
            D::E => if self.pos.x == w - 1 { None } else { Some(Pos{x: self.pos.x + 1, y: self.pos.y}) }
            D::W => if self.pos.x == 0 { None } else { Some(Pos{x: self.pos.x - 1, y: self.pos.y}) }
            D::N => if self.pos.y == 0 { None } else { Some(Pos{x: self.pos.x, y: self.pos.y - 1}) }
            D::S => if self.pos.y == h - 1 { None } else { Some(Pos{x: self.pos.x, y: self.pos.y + 1}) }
        }
    }

    fn try_move(&mut self, board: &Vec<Vec<char>>, w: usize, h: usize)-> bool {
        let maybe_next_pos = self.next_pos(w, h);
        // println!("{:?}", maybe_next_pos);
        if maybe_next_pos.is_none() {
            return false
        } else {
            let next_pos = maybe_next_pos.unwrap();
            match self.d {
                D::E => {
                    if board[next_pos.y][next_pos.x] == '#' {
                        self.d = D::S;
                    } else {
                        self.pos = next_pos;
                    }
                },
                D::W => {
                    if board[next_pos.y][next_pos.x] == '#' {
                        self.d = D::N;
                    } else {
                        self.pos = next_pos;
                    }
                },
                D::N => {
                    if board[next_pos.y][next_pos.x] == '#' {
                        self.d = D::E;
                    } else {
                        self.pos = next_pos;
                    }
                },
                D::S => {
                    if board[next_pos.y][next_pos.x] == '#' {
                        self.d = D::W;
                    } else {
                        self.pos = next_pos;
                    }
                }
            }
            true
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

fn traverse(board: &Vec<Vec<char>>, mut guard: Guard, w: usize, h: usize) -> HashSet<Pos> {
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(guard.pos);

    while guard.try_move(&board, w, h) {
        visited.insert(guard.pos);
    }

    visited
}

fn is_loop(board: &Vec<Vec<char>>, mut guard: Guard, w: usize, h: usize) -> bool {
    let mut visited_and_dir: HashSet<(Pos, D)> = HashSet::new();
    visited_and_dir.insert((guard.pos, guard.d));

    while guard.try_move(&board, w, h) {
        if visited_and_dir.contains(&(Pos{x: guard.pos.x, y: guard.pos.y}, guard.d)) {
            return true
        }
        visited_and_dir.insert((guard.pos, guard.d));
    }

    false
}


fn traverse_loop(visited: HashSet<Pos>, mut board: Vec<Vec<char>>, guard: Guard, w: usize, h: usize) -> i32 {
    let mut loop_count = 0;
    let start_pos = guard.pos.clone();
    for pos in visited.iter() {
        if *pos == start_pos {
            // skip starting spot
            continue
        }

        // pretend there is an obstacle on the board and run through to see if the guard loops
        board[pos.y][pos.x] = '#';

        if is_loop(&board, guard.clone(), w, h) {
            loop_count += 1;
        }

        board[pos.y][pos.x] = '.';
    }

    loop_count
}

fn main() {
    let (guard, board) = load_input("in");

    let h = board.len();
    let w = board.first().unwrap().len();

    let visited = traverse(&board, guard.clone(), w, h);
    println!("{:?}", visited.len());

    let p2 = traverse_loop(visited, board.clone(), guard.clone(), w, h);
    println!("{:?}", p2);
}