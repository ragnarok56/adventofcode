use std::collections::HashMap;
use std::{fs::File};
use std::io::{BufReader, BufRead};
use topological_sort::TopologicalSort;


#[derive(Clone, Debug)]
struct Monkey {
    id: String,
    start_number: Option<i64>,
    number: Option<i64>,
    operator: Option<String>,
    left: Option<String>,
    right: Option<String>
}

fn is_root_equal(monkey_map: &HashMap<String, Monkey>) -> bool {
    let root = monkey_map.get("root").unwrap();
    let left = monkey_map.get(root.left.as_ref().unwrap()).unwrap();
    let right = monkey_map.get(root.right.as_ref().unwrap()).unwrap();
    if left.number.is_none() || right.number.is_none() {
        return false
    }
    let left_number = left.number.unwrap();
    let right_number = right.number.unwrap();
    // println!("left: {}, right: {}, diff: {}", left_number, right_number, left_number - right_number);
    return left_number == right_number;
}

fn get_root_diff(monkey_map: &HashMap<String, Monkey>) -> Option<i64> {
    let root = monkey_map.get("root").unwrap();
    let left = monkey_map.get(root.left.as_ref().unwrap()).unwrap();
    let right = monkey_map.get(root.right.as_ref().unwrap()).unwrap();
    if left.number.is_none() || right.number.is_none() {
        return None
    }
    let left_number = left.number.unwrap();
    let right_number = right.number.unwrap();
    return Some(left_number - right_number);
}


fn part(part2: bool) {
    let file = File::open("in").expect("file doesnt exist");

    let reader = BufReader::new(file);

    let binding = reader.lines().map(|x| x.unwrap());
    let monkeys: Vec<Monkey> = binding.map(|x| {
        let mut row = x.split(":");
        let id = row.next().unwrap().trim_end();
        let value = row.next().unwrap().trim();
        let number = value.parse::<i64>().ok();
        if number.is_some() {
            return Monkey{id: id.to_string(), start_number: number, number: None, left: None, operator: None, right: None}
        }
        let mut expression = value.split_whitespace();
        let left = expression.next().map(|s| s.to_string());
        let operator = expression.next().map(|s| s.to_string());
        let right = expression.next().map(|s| s.to_string());
        Monkey{id: id.to_string(), start_number: None, number: None, left: left, operator: operator, right: right}
    }).collect();


    let mut monkey_map: HashMap<String, Monkey> = monkeys.into_iter().map(|x| (x.id.clone(), x)).collect();

    let mut ts = TopologicalSort::<String>::new();
    for m in monkey_map.values() {
        if m.start_number.is_none() {
            ts.add_dependency(m.left.as_ref().unwrap(), m.id.to_string());
            ts.add_dependency(m.right.as_ref().unwrap(), m.id.to_string());
        }
    }
    let ts_orig = ts.clone();
    if part2 {
        // strategy for part2 is to hardcode the human values to increment or decrement 
        // and see what happens
        // lots of trial and error to get down to the match
        // im sure there is a trick that involves parsing the input or the formulas
        // but this took like, 15 minutes to brute force
        monkey_map.entry("humn".to_string()).and_modify(|e| e.start_number = Some(3378273370614));
    }

    let mut start_seesawing = false;
    let mut check_interval = 10000000;
    if !part2 {
        check_interval = 0;
    }

    while !part2 || !is_root_equal(&monkey_map) {
        monkey_map.entry("humn".to_string()).and_modify(|e| e.start_number = Some(e.start_number.unwrap() + check_interval));
        ts = ts_orig.clone();
        while !ts.is_empty() {
            let batch = ts.pop_all();
            for v in batch {
                let m = monkey_map.get(&v).unwrap();
                if m.start_number.is_none() {
                    let left_monkey = monkey_map.get(m.left.as_ref().unwrap()).unwrap();
                    let right_monkey = monkey_map.get(m.right.as_ref().unwrap()).unwrap();
                    let left = left_monkey.start_number.unwrap_or_else(|| left_monkey.number.unwrap());
                    let right = right_monkey.start_number.unwrap_or_else(|| right_monkey.number.unwrap());
                    let number = match m.operator.as_ref().unwrap().as_str() {
                        "+" => Some(left + right),
                        "-" => Some(left - right),
                        "*" => Some(left * right),
                        "/" => Some(left / right),
                        _ => None
                    };
                    monkey_map.entry(v).and_modify(|e| e.number = number);
                }
            }
        }
        let diff = get_root_diff(&monkey_map);
        if start_seesawing || diff.unwrap() < 0 {
            // just start narrowing down the potential numbers once we know we've gone past it
            start_seesawing = true;
            if check_interval < 0 {
                check_interval = (check_interval + 1) * -1;
            } else {
                check_interval = (check_interval - 1) * -1;
            }
        }

        if !part2 {
            println!("{:?}", monkey_map.get("root").unwrap().number.unwrap());
            break
        }
    }
    
    if part2 {
        println!("human number: {}", monkey_map.get("humn").unwrap().start_number.unwrap());
    }
}

fn main() {
    part(false);
    part(true);
}
