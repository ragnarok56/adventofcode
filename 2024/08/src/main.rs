use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn is_on_board(&self, w: usize, h: usize) -> bool {
        self.x < w && self.y < h
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct Antenna {
    pos: Pos,
    signal: char
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Copy)]
struct AntennaRelation {
    antenna: Antenna,
    distance: usize
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

fn get_next_pos(cur: Pos, x: i32, y: i32) -> Option<Pos> {
    let x =
        if x.is_negative() {
            cur.x.checked_sub(x.abs() as usize)
        } else {
            cur.x.checked_add(x as usize)
        };
    let y =
        if y.is_negative() {
            cur.y.checked_sub(y.abs() as usize)
        } else {
            cur.y.checked_add(y as usize)
        };
    if y.is_none() || x.is_none() {
        return None
    }
    Some(Pos{x: x.unwrap(), y: y.unwrap()})
}

fn print_board(board: &Vec<Vec<char>>, antinodes: HashMap<&Pos, &char>) {

    for (i, r) in board.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            let antinode: Option<&&char> = antinodes.get(&Pos{x: j, y: i});
            if antinode.is_some() {
                print!("{}", antinode.unwrap());
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn main() {
    let (antennas, board) = load_input("in_test");

    let h = board.len();
    let w = board.first().unwrap().len();

    let signals: HashSet<char> = antennas.iter().fold(HashSet::new(), |mut acc, x| {
        acc.insert(x.signal);
        acc
    });

    println!("{:?}", signals);
    println!("{:?}", antennas);

    let mut signal_tracker: HashMap<Pos, HashMap<char, Vec<AntennaRelation>>> = HashMap::new();
    let moves: Vec<(i32, i32)> = vec![
        (-1, 0), // W
        (-1, -1), // NW
        (0, -1), // N
        (1, -1), // NE
        (1, 0), // E
        (1, 1), // SE
        (0, 1), // S
        (-1, 1) // SW
    ];

    for antenna in antennas {
        println!("Checking {:?}", antenna);
        for m in moves.iter() {
            let mut signal_dir_pos = get_next_pos(antenna.pos, m.1, m.0);
            while signal_dir_pos.is_some() && signal_dir_pos.unwrap().is_on_board(w, h) {
                println!("{:?}", signal_dir_pos);
                let signal_pos = signal_dir_pos.unwrap();

                let distances = vec![antenna.pos.x.abs_diff(signal_pos.x), antenna.pos.y.abs_diff(signal_pos.y)];
                let distance = distances.iter().max().unwrap();
                println!("A {:?}, S {:?}, D {:?}", antenna.pos, signal_pos, distance);

                signal_tracker
                    .entry(signal_pos)
                    .or_insert(HashMap::new())
                        .entry(antenna.signal)
                        .or_insert(Vec::new())
                        .push(AntennaRelation{antenna: antenna, distance: *distance});

                signal_dir_pos = get_next_pos(signal_dir_pos.unwrap(), m.1, m.0);
            }
        }
    }

    let mut sum = 0;
    let mut antinode_positions = HashMap::new();
    for (pos, s) in signal_tracker.iter() {
        let is_antinode_pos = s
            .iter()
            .filter(|x| x.1.len() > 1)
            .filter(|(signal, ars)| {
                ars
                    .iter()
                    .any(|ar| {
                        ars
                            .iter()
                            .find(|other_ar| ar.distance * 2 == other_ar.distance).is_some()
                })
            }).next();
        if is_antinode_pos.is_some() {
            println!("{:?}, {:?}", pos, is_antinode_pos);
            antinode_positions.insert(pos, is_antinode_pos.unwrap().0);
            sum += 1;
        }
    }
    print_board(&board, antinode_positions);
    println!("{:?}", sum);
}