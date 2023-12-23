use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum D {
    N, S, E, W
}

#[derive(Debug)]
struct Beam {
    id: i32,
    pos: Pos,
    d: D
}

impl Beam {
    fn try_move_beam(&mut self, w: usize, h: usize)-> bool {

        match self.d {
            D::E => {
                if self.pos.col == w - 1 {
                    false
                } else {
                    self.pos.col += 1;
                    true
                }
            },
            D::W => {
                if self.pos.col == 0 {
                    false
                } else {
                    self.pos.col -= 1;
                    true
                }
            },
            D::N => {
                if self.pos.row == 0 {
                    false
                } else {
                    self.pos.row -= 1;
                    true
                }
            },
            D::S => {
                if self.pos.row == h - 1 {
                    false
                } else {
                    self.pos.row += 1;
                    true
                }
            }
        }
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize
}

struct Light {
    cur_id: i32
}

impl Light {
    fn create_beam(&mut self) -> Beam {
        let beam = Beam { id: self.cur_id, pos: Pos { row: 0, col: 0 }, d: D::E };
        self.cur_id += 1;
        beam
    }
}

#[allow(dead_code)]
fn display_matrix(matrix: &Vec<Vec<char>>, energized: &HashSet<Pos>, show_energized: bool) {
    for (row, r) in matrix.iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            print!("{}", if show_energized && energized.contains(&Pos{ row, col }) { '#' } else { *c });
        }
        println!("");
    }
}

fn light_it_up(matrix: &Vec<Vec<char>>, start: Pos, direction: D) -> HashSet<Pos> {

    let w = matrix[0].len();
    let h = matrix.len();

    let mut energized: HashSet<Pos> = HashSet::new();

    let mut light = Light { cur_id: 0 };

    let mut active_beams: Vec<Beam> = Vec::new();
    let mut initial_beam = light.create_beam();
    initial_beam.pos = start;
    initial_beam.d = direction;
    active_beams.push(initial_beam);

    let mut visited: HashSet<(D, Pos)> = HashSet::new();
    while active_beams.len() > 0 {
        let mut beams_to_remove: Vec<i32> = Vec::new();
        let mut beams_to_add: Vec<Beam> = Vec::new();
        for beam in active_beams.iter_mut() {
            let c = matrix.get(beam.pos.row).unwrap().get(beam.pos.col).unwrap();
            energized.insert(beam.pos);
            if visited.contains(&(beam.d, beam.pos)) {
                beams_to_remove.push(beam.id);
                continue;
            }
            visited.insert((beam.d, beam.pos));
            match c {
                '|' => {
                    if beam.d == D::E || beam.d == D::W {
                        beam.d = D::N;
                        let mut split_beam = light.create_beam();
                        split_beam.pos = beam.pos.clone();
                        split_beam.d = D::S;
                        beams_to_add.push(split_beam);
                    }
                },
                '-' => {
                    if beam.d == D::N || beam.d == D::S {
                        beam.d = D::E;
                        let mut split_beam = light.create_beam();
                        split_beam.pos = beam.pos.clone();
                        split_beam.d = D::W;
                        beams_to_add.push(split_beam);
                    }
                },
                '/' => {
                    match beam.d {
                        D::E => beam.d = D::N,
                        D::W => beam.d = D::S,
                        D::S => beam.d = D::W,
                        D::N => beam.d = D::E
                    }
                },
                '\\' => {
                    match beam.d {
                        D::E => beam.d = D::S,
                        D::W => beam.d = D::N,
                        D::S => beam.d = D::E,
                        D::N => beam.d = D::W
                    }
                }
                _ => ()
            }

            let can_move = beam.try_move_beam(w, h);
            if !can_move {
                beams_to_remove.push(beam.id);
            }
        }
        for beam_id in beams_to_remove.iter() {
            let beam_idx = active_beams.iter().position(|x| x.id == *beam_id);
            if beam_idx.is_some() {
                active_beams.remove(beam_idx.unwrap());
            }
        }
        active_beams.append(&mut beams_to_add);
        beams_to_remove.clear();
        beams_to_add.clear();
    }

    energized
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();
    let matrix: Vec<Vec<char>> = lines
        .map(|x| {
            x.chars()
                .fold(Vec::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                })
        })
        .collect();

    let p1 = light_it_up(&matrix, Pos { row: 0, col: 0 }, D::E);
    println!("p1: {}", p1.len());

    let mut p2 = 0;
    let w = matrix.len();
    let h = matrix[0].len();
    let corner_cases =  &[
        (0, 0, D::E),
        (0, h - 1, D::W),
        (w - 1, 0, D::E),
        (w - 1, h - 1, D::W)
    ];
    for (row, col, d) in corner_cases {
        let energized = light_it_up(&matrix, Pos { row: *row, col: *col }, *d);
        let count = energized.len();
        if count > p2 {
            p2 = count;
        }
    }
    for (row, d) in &[(0, D::S), (h - 1, D::N)] {
        for col in 0..w {
            let energized = light_it_up(&matrix, Pos { row: *row, col }, *d);
            let count = energized.len();
            if count > p2 {
                p2 = count;
            }
        }
    }
    for (col, d) in &[(0, D::E), (w - 1, D::W)] {
        for row in 0..h {
            let energized = light_it_up(&matrix, Pos { row, col: *col }, *d);
            let count = energized.len();
            if count > p2 {
                p2 = count;
            }
        }
    }

    println!("p2: {}", p2);
}