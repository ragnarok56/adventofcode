use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};


#[derive(Copy, Clone, Debug)]
struct Instr {
    cycles: i32,
    value: i32
}

impl Instr {
    fn is_complete(self) -> bool {
        return self.cycles <= 0;
    }

    fn process(&mut self) {
        self.cycles = self.cycles - 1;
    }
}

fn create_instr(command: &String) -> Option<Instr> {
    let mut instr_split = command.split(" ");
    let instr = instr_split.next().unwrap();
    return match instr {
        "noop" => Some(Instr { value: 0, cycles: 1 }),
        "addx" => Some(Instr { value: instr_split.next().unwrap().parse().unwrap(), cycles: 2 }),
        _ => None
    };
}

fn main() {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|x| x.unwrap());

    // initialize stuff
    let mut command = lines.next();
    let mut cycle = 1;
    let mut register: i32 = 1;
    let mut instr = create_instr(command.as_ref().unwrap()).unwrap();
    let mut screen = [[false; 40]; 6];

    let cycles_to_track = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut result = 0;

    command = lines.next();

    while command.is_some() {
        if instr.is_complete() {
            register = register + instr.value;

            let new_instr = create_instr(command.as_ref().unwrap());
            if new_instr.is_some() {
                instr = new_instr.unwrap();
            }

            command = lines.next();
        }
        instr.process();

        if cycles_to_track.contains(&cycle) {
            result = result + (cycle * register);
        }

        let row: usize = ((cycle - 1) / 40) as usize;
        let col: usize = ((cycle - 1) % 40 )as usize;
        screen[row][col] = (register - (col as i32)).abs() < 2;

        cycle = cycle + 1;
    }
    println!("signal strength sum: {}", result);
    for row in screen {
        for col in row {
            print!("{}", if col { "#" } else { "." });
        }
        println!();
    }

}
