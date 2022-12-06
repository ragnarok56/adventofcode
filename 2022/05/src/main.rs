use std::fs::File;
use std::io::{self, BufReader, BufRead};

#[derive(Debug)]
struct Crate {
    content: char
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>
}

fn part1() {
    /*
                        [Z] [W] [Z]
        [D] [M]         [L] [P] [G]
    [S] [N] [R]         [S] [F] [N]
    [N] [J] [W]     [J] [F] [D] [F]
[N] [H] [G] [J]     [H] [Q] [H] [P]
[V] [J] [T] [F] [H] [Z] [R] [L] [M]
[C] [M] [C] [D] [F] [T] [P] [S] [S]
[S] [Z] [M] [T] [P] [C] [D] [C] [D]
 1   2   3   4   5   6   7   8   9  */
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut crate_input_lines: Vec<String> = lines
        .by_ref()
        .take_while(|x| x.as_ref().unwrap().len() > 0)
        .map(|x| x.unwrap().to_string())
        .collect();

    crate_input_lines.reverse();
    println!("{:?}", crate_input_lines);
    let num_stacks = crate_input_lines
        .get(0)
        .unwrap()
        .replace(" ", "")
        .pop()
        .unwrap().to_string().parse().unwrap();

    let mut stacks = Vec::new();
    {0..num_stacks}.for_each(|_| {
        stacks.push(Stack { crates: Vec::new() });
    });

    crate_input_lines
        .iter()
        .skip(1)
        .for_each(|x| {
            x
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .filter(|c| *c.1.get(0).unwrap() == '[')
                .for_each(|c| {
                    let crate_to_add = Crate { content: *c.1.get(1).unwrap() };
                    stacks
                        .get_mut(c.0)
                        .unwrap()
                        .crates
                        .push(crate_to_add);
                })
        });

    // println!("{:?}, {:?}", stacks, stacks.len());

    lines
        .map(|x| x.unwrap())
        .for_each(|x| {
            println!("{:?}", x);
            let mut moves = x.split(" ");
            let num_crates_to_move: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_from_index: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_to_index: usize = moves.nth(1).unwrap().parse().unwrap();
            println!("{:?}-{:?}-{:?}", num_crates_to_move, stack_from_index, stack_to_index);
            {0..num_crates_to_move}.for_each(|_| {
                let crate_to_move = stacks
                    .get_mut(stack_from_index - 1)
                    .unwrap()
                    .crates.pop().unwrap();
                stacks
                    .get_mut(stack_to_index - 1)
                    .unwrap()
                    .crates.push(crate_to_move);
            })
        });

    stacks.iter_mut().for_each(|x| {
        print!("{}", x.crates.pop().unwrap().content);
    })

}

fn main() {
    part1();
    part2();
}