use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Copy, Clone, Debug)]
struct Crate {
    content: char
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>
}

fn parse_input() -> (Vec<Stack>, Vec<String>) {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut crate_input_lines: Vec<String> = lines
        .by_ref()
        .take_while(|x| x.as_ref().unwrap().len() > 0)
        .map(|x| x.unwrap().to_string())
        .collect();

    crate_input_lines.reverse();

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

    let moves: Vec<String> = lines
        .map(|x| x.unwrap())
        .collect();

    return (stacks,moves)

}

fn part1() {
    let (mut stacks, moves) = parse_input();
    moves
        .iter()
        .for_each(|x| {
            let mut moves = x.split(" ");
            let num_crates_to_move: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_from_index: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_to_index: usize = moves.nth(1).unwrap().parse().unwrap();

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
    });
    println!()

}


fn part2() {
    let (mut stacks, moves) = parse_input();
    moves
        .iter()
        .for_each(|x| {
            let mut moves = x.split(" ");
            let num_crates_to_move: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_from_index: usize = moves.nth(1).unwrap().parse().unwrap();
            let stack_to_index: usize = moves.nth(1).unwrap().parse().unwrap();

            let mut crates_to_move: Vec<Crate> = {0..num_crates_to_move}
                .map(|_| {
                    stacks
                        .get_mut(stack_from_index - 1)
                        .unwrap()
                        .crates.pop().unwrap()
                })
                .collect();
            crates_to_move.reverse();
            crates_to_move.iter().for_each(|c| {
                stacks
                    .get_mut(stack_to_index - 1)
                    .unwrap()
                    .crates.push(*c);
            })
        });

    stacks.iter_mut().for_each(|x| {
        print!("{}", x.crates.pop().unwrap().content);
    });
    println!()

}

fn main() {
    part1();
    part2();
}