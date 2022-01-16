use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum AmphipodType {
    A,
    B,
    C,
    D
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Amphipod {
    id: usize,
    pod_type: AmphipodType
}

#[derive(Debug)]
struct Board {
    rooms: Vec::<Vec::<usize>>,
    hallway: HashMap::<usize, usize>
}

impl Board {
    fn print(&self, amphipods: HashMap<usize, Amphipod>) {
        println!("{:#<1$}", "", 13);
        print!("#");
        for i in 0..11 {
            if self.hallway.contains_key(&i) {
                let amphipod_id = self.hallway.get(&i).unwrap();
                print!("{:?}", amphipods.get(amphipod_id).unwrap().pod_type);
            } else {
                print!(".")
            }
        }
        print!("#");
        println!();
        print!("###");
        for r in self.rooms.iter() {
            print!("{:?}#", r[0]);
        }
        println!("##");
        print!("  #");
        for r in self.rooms.iter() {
            print!("{:?}#", r[1]);
        }
        println!();
        println!("  {:#<1$}  ", "", 9);
    }
}

fn setup_amphipods() -> HashMap<usize, Amphipod> {
    let amphipods_types = vec![
        AmphipodType::A, AmphipodType::A,
        AmphipodType::B, AmphipodType::B,
        AmphipodType::C, AmphipodType::C,
        AmphipodType::D, AmphipodType::D
    ];

    let amphipods = amphipods_types.iter().enumerate()
        .fold(HashMap::<usize, Amphipod>::new(), |mut acc, (i, x)| {
            let amphipod = Amphipod{id: i, pod_type: x.clone()};
            acc.entry(i).or_insert(amphipod);
            acc
        });
    
    amphipods
}

fn setup_board(amphipods: &HashMap<usize, Amphipod>) -> Board {
    let mut board = Board{
        rooms: vec![Vec::<usize>::new(); 4],
        hallway: HashMap::<usize, usize>::new(),
    };

    let mut pq = PriorityQueue::new();
    for (_id, x) in amphipods.iter() {
        let pod_type = x.pod_type.clone();
        pq.push(x, pod_type);
    }

    println!("{:?}", board.rooms);
    
    // starting state
    // #############
    // #...........#
    // ###D#B#C#A###
    //   #C#A#D#B#
    //   #########

    board.rooms[0].push(7);
    board.rooms[0].push(5);
    board.rooms[1].push(3);
    board.rooms[1].push(1);
    board.rooms[2].push(4);
    board.rooms[2].push(6);
    board.rooms[3].push(0);
    board.rooms[3].push(2);

    board
}

fn main() {
    let amphipods = setup_amphipods();

    let board = setup_board(&amphipods);

    board.print(amphipods);
}