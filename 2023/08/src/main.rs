use core::panic;
use std::{fs, collections::HashMap};



#[derive(Debug)]
struct Node {
    steps: u64,
    node: String,
    found: bool
}

// totally stolen from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let nodes = lines.map(|x| {
        let mut split = x.split(" = ");
        let node = split.next().unwrap();
        let replaced_split = split.next().unwrap().trim_matches(&['(', ')'][..]);
        let mut adj_nodes = replaced_split.split(", ");
        let left = adj_nodes.next().unwrap();
        let right = adj_nodes.next().unwrap();
        (node, (left, right))
    })
    .collect::<Vec<_>>();

    let map: HashMap::<&str, (&str, &str)> = HashMap::from_iter(nodes);

    let mut cur = "AAA";
    let mut p1 = 0;
    let mut found = false;
    while !found {
        for d in directions.iter() {
            match d {
                'L' => cur = map.get(cur).unwrap().0,
                'R' => cur = map.get(cur).unwrap().1,
                _ => panic!()
            }
            p1 += 1;
            if cur == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    println!("p1: {:?}", p1);

    found = false;
    let mut cur_nodes: Vec<Node> = map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| Node { node: x.to_string(), steps: 0, found: false })
        .collect::<Vec<_>>();

    while !found {
        for d in directions.iter() {
            for cur in cur_nodes.iter_mut().filter(|x| !x.found) {
                match d {
                    'L' => cur.node = map.get(cur.node.as_str()).unwrap().0.to_string(),
                    'R' => cur.node = map.get(cur.node.as_str()).unwrap().1.to_string(),
                    _ => panic!()
                }
                cur.steps += 1;
                if cur.node.ends_with("Z") {
                    cur.found = true;
                }
            }
            if cur_nodes.iter().all(|x| x.found) {
                found = true;
                break;
            }
        }
    }

    let step_counts: Vec<u64> = cur_nodes.iter().map(|x| x.steps).collect::<Vec<_>>();
    let p2 = lcm(step_counts.as_slice());

    println!("p2: {:?}", p2);
}