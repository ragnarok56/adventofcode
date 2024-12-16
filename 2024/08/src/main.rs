use std::fs;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn get_antinode(&self, other: Pos, w: usize, h: usize) -> Option<Pos> {
        let x = self.x as i32 - other.x as i32;
        let y = self.y as i32 - other.y as i32;

        let antinode_x =
            if x.is_negative() {
                self.x.checked_sub(x.abs() as usize)
            } else {
                self.x.checked_add(x as usize)
            };
        let antinode_y =
            if y.is_negative() {
                self.y.checked_sub(y.abs() as usize)
            } else {
                self.y.checked_add(y as usize)
            };

        if antinode_x.is_none_or(|x| x >= w) || antinode_y.is_none_or(|y| y >= h) {
            return None;
        }

        return Some(Pos{x: antinode_x.unwrap(), y: antinode_y.unwrap()})
    }

    // fn get_antinodes(&self, other: Pos, w: usize, h: usize) -> Option<Pos> {
    //     // for part 2, try recursively calling get_antinodes each time one is found
    //     None
    // }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct Antenna {
    pos: Pos,
    signal: char
}


fn load_input(path: &str) -> (Vec<Antenna>, Vec<Vec<char>>) {
    let mut antennas = Vec::new();
    let board = fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, x)| {
            x.chars().enumerate().fold(Vec::new(), |mut acc, (j, x)| {
                if x != '.' {
                    antennas.push(Antenna{pos: Pos{x: j, y: i}, signal: x})
                }
                acc.push(x);
                acc
            })
        })
        .collect();

    (antennas, board)
}

// fn print_board(board: &Vec<Vec<char>>, antinodes: &HashSet<Pos>) {

//     for (i, r) in board.iter().enumerate() {
//         for (j, c) in r.iter().enumerate() {
//             let antinode = antinodes.get(&Pos{x: j, y: i});
//             if antinode.is_some() {
//                 print!("#");
//             } else {
//                 print!("{}", c);
//             }
//         }
//         println!();
//     }
// }

fn main() {
    let (antennas, board) = load_input("in");

    let h = board.len();
    let w = board.first().unwrap().len();

    let antenna_groups = antennas.iter().fold(HashMap::new(), |mut acc, antenna| {
        acc.entry(antenna.signal).or_insert(Vec::new()).push(antenna);
        acc
    });

    let mut antinode_pos_set = HashSet::new();
    for (_, ans) in antenna_groups.iter() {
        for pair in ans.iter().combinations(2) {
            let mut pair_iter = pair.iter();
            let first = pair_iter.next().unwrap();
            let second = pair_iter.next().unwrap();
            let first_antinode = first.pos.get_antinode(second.pos, w, h);
            let second_antinode = second.pos.get_antinode(first.pos, w, h);
            if first_antinode.is_some() {
                antinode_pos_set.insert(first_antinode.unwrap());
            }
            if second_antinode.is_some() {
                antinode_pos_set.insert(second_antinode.unwrap());
            }
        }
    }

    // print_board(&board, &antinode_pos_set);

    println!("{:?}", antinode_pos_set.len());

}