use std::{fs, collections::HashMap};
use pathfinding::prelude::astar;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    val: i32,
    pos: Pos
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct NodePos {
    pos: Pos,
    dir: Option<D>,
    dir_count: i32
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum D {
    N, S, E, W
}

impl NodePos {
    fn get_successors(self: &NodePos, matrix: &Vec<Vec<Node>>, min_moves: Option<i32>, max_moves: i32) -> Vec<(NodePos, i32)> {
        let mut invalid_dirs = HashMap::new();
        invalid_dirs.insert(D::N, D::S);
        invalid_dirs.insert(D::S, D::N);
        invalid_dirs.insert(D::W, D::E);
        invalid_dirs.insert(D::E, D::W);

        let mut result:Vec<(NodePos, i32)> = Vec::new();
        let h = matrix.len() as i32;
        let w = matrix.get(0).unwrap().len() as i32;
        for (r, c, dir) in &[(-1, 0, D::N), (1, 0, D::S), (0, 1, D::E), (0, -1, D::W)] {
            // check bounds
            if (self.pos.row as i32) + r < 0 {
                continue
            }
            if (self.pos.row as i32) + r > (h - 1) {
                continue
            }
            if (self.pos.col as i32) + c < 0 {
                continue
            }
            if (self.pos.col as i32) + c > (w - 1) {
                continue
            }
            let mut row = self.pos.row;
            let mut col = self.pos.col;
            if *r < 0 {
                row -= 1;
            } else if *r > 0 {
                row += 1;
            }
            if *c < 0 {
                col -= 1;
            } else if *c > 0 {
                col += 1;
            }
            let mut dir_count = 1;
            if self.dir.is_some() {
                // dont go backward to get around move restrictions
                // this could still cause a cycle but hoping the A* routing solves that
                if invalid_dirs.get(&self.dir.unwrap()).unwrap() == dir {
                    continue;
                }

                if self.dir.unwrap() == *dir {
                    // same direction, so keep dir_count going from previous node
                    dir_count += self.dir_count;
                } else {
                    // different direction, only eligible if we currently moved more than min_moves, if thats set
                    if min_moves.is_some() && self.dir_count < min_moves.unwrap() {
                        continue;
                    }
                }
                // never move more than max moves
                if dir_count > max_moves {
                    continue;
                }
            }
            result.push((NodePos { pos: Pos { row, col }, dir_count, dir: Some(*dir)}, matrix.get(row).unwrap().get(col).unwrap().val))
        }
        result
    }
}

#[allow(dead_code)]
fn display_path(matrix: &Vec<Vec<Node>>, path: &Vec<NodePos>) {
    for r in matrix.iter() {
        for c in r.iter() {
            if path.iter().any(|x| x.pos == c.pos) {
                print!("#");
            }
            else {
                print!("{}", c.val);
            }
        }
        println!();
    }
}


fn main() {
    let input = fs::read_to_string("in_test").unwrap();
    let lines = input.lines();
    let matrix: Vec<Vec<Node>> = lines
        .enumerate()
        .map(|(row, x)| {
            x.chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (col, c)| {
                    acc.push(Node { val: c.to_digit(10).unwrap() as i32, pos: Pos { row, col }});
                    acc
                })
        })
        .collect();

    let start = NodePos { pos: matrix.get(0).unwrap().get(0).unwrap().pos, dir_count: 0, dir: None };

    let goal_row = matrix.len() - 1;
    let goal_col = matrix.get(0).unwrap().len() - 1;
    let goal_pos = Pos { row: goal_row, col: goal_col };

    let r = astar(&start,
        |n| n.get_successors(&matrix, None, 3),
        |_| 0, // throw out estimation
        |n| n.pos == goal_pos);
    if r.is_some() {
        let mut path = r.unwrap();
        path.0.remove(0);

        let p1: i32 = path.0.iter().map(|x| matrix.get(x.pos.row).unwrap().get(x.pos.col).unwrap().val).sum();
        println!("p1: {:?}", p1);
    }

    let r = astar(&start,
        |n| n.get_successors(&matrix, Some(4), 10),
        |_| 0, // throw out estimation
        |n| n.pos == goal_pos);
    if r.is_some() {
        let mut path = r.unwrap();
        path.0.remove(0);

        let p2: i32 = path.0.iter().map(|x| matrix.get(x.pos.row).unwrap().get(x.pos.col).unwrap().val).sum();
        println!("p2: {:?}", p2);
    }
}
