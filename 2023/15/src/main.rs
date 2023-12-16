use std::fs;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: String
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |mut acc, c| {
        acc = ((acc + (c as u32)) * 17) % 256;
        acc
    })
}

fn parse_lens(s: &str) -> Lens {
    let mut split = s.split(&['=', '-']);
    let label = split.next().unwrap().to_string();
    let focal_length = split.next().unwrap().to_string();
    Lens { label, focal_length }
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();
    let line = lines.collect::<String>();

    let p1: u32 = line.split(',')
        .map(|x| hash(x))
        .sum();

    println!("p1: {}", p1);
    
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    line.split(',')
        .for_each(|x| {
            let lens = parse_lens(x);
            let idx = hash(&lens.label) as usize;
            let label_idx = boxes[idx].iter().position(|l| *l.label == lens.label);
            if label_idx.is_some() {
                boxes[idx].remove(label_idx.unwrap());
            }
            if x.contains('=') {
                if label_idx.is_none() {
                    boxes[idx].push(lens);
                } else {
                    boxes[idx].insert(label_idx.unwrap(), lens);
                }
            }
        });
        
    let p2: usize = boxes.iter().enumerate()
        .map(|(i, x)| {
            let box_power: usize = x.iter().enumerate().map(|(j, lens)| {
                (j + 1) * lens.focal_length.parse::<usize>().unwrap()
            }).sum();
            (i + 1) * box_power
        })
        .sum();
    println!("p2: {}", p2);
}