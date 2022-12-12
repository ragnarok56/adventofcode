// use std::fs::File;
use std::collections::HashMap;
// use std::io::{BufReader, BufRead};


#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: fn(i64) -> i64,
    test: fn(i64) -> i64,
    divisible: i64
}

fn parts(is_part_1: bool) {
    
    // let file = File::open("in").expect("file doesnt exist");

    // let reader = BufReader::new(file);

    // let monkeys: Vec<Monkey> = reader.lines()
    //     .map(|x| x.unwrap())
    //     .collect::<Vec<String>>()
    //     .chunks(5)
    //     .fold(Vec::new(), |mut acc, x| {
    //         let items: Vec<i64> = x[0].split(":").skip(1).next().unwrap().split(",").map(|x| x.trim().parse().unwrap()).collect();
    //         let op_end = x[1].split(":").skip(1).next().unwrap().split("=").skip(1).next().unwrap();
    //         let ops = op_end.split(" ").skip(1);
    //         let operation = ops.next().unwrap();
    //         let value = ops.next().unwrap();

    //         let op_closure: fn(i64) -> i64;
    //         if operation == "+" {
    //             op_closure = |x: i64| {
    //                 match value {
    //                     "old" => x + x,
    //                     _ => x + value.parse::<i64>().unwrap()
    //                 }
    //             }
            
    //         match operation {
    //             "+" => |x: i64| {
    //                 match value {
    //                     "old" => x + x,
    //                     _ => x + value.parse::<i64>().unwrap()
    //                 }
    //             },
    //             _ => |x: i64| {
    //                 match value {
    //                     "old" => x * x,
    //                     _ => x * value.parse::<i64>().unwrap()
    //                 }
    //             }
    //         }
    //         acc.push(Monkey { items: Vec::new(), op: |x| x, test: |x| x });
    //         acc
    //     })
    //     .into_iter()
    //     .collect::<Vec<Monkey>>();

    // let l: Vec<String> = lines
    //     .filter(|x| x.len() > 0 && !x.starts_with("M"))
    //     .collect();
    // l.chunks()
    // println!("{:?}", l);

    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey { 
        items: vec![50, 70, 89, 75, 66, 66],
        op: |x| x * 5,
        test: |x| if x % 2 == 0 { 2 } else { 1 },
        divisible: 2
    });
    monkeys.push(Monkey { 
        items: vec![85],
        op: |x| x * x,
        test: |x| if x % 7 == 0 { 3 } else { 6 },
        divisible: 7
    });
    monkeys.push(Monkey { 
        items: vec![66, 51, 71, 76, 58, 55, 58, 60],
        op: |x| x + 1,
        test: |x| if x % 13 == 0 { 1 } else { 3 },
        divisible: 13
    });
    monkeys.push(Monkey { 
        items: vec![79, 52, 55, 51],
        op: |x| x + 6,
        test: |x| if x % 3 == 0 { 6 } else { 4 },
        divisible: 3
    });
    monkeys.push(Monkey { 
        items: vec![69, 92],
        op: |x| x * 17,
        test: |x| if x % 19 == 0 { 7 } else { 5 },
        divisible: 19
    });
    monkeys.push(Monkey { 
        items: vec![71, 76, 73, 98, 67, 79, 99],
        op: |x| x + 8,
        test: |x| if x % 5 == 0 { 0 } else { 2 },
        divisible: 5
    });
    monkeys.push(Monkey { 
        items: vec![82, 76, 69, 69, 57],
        op: |x| x + 7,
        test: |x| if x % 11 == 0 { 7 } else { 4 },
        divisible: 11
    });
    monkeys.push(Monkey { 
        items: vec![65, 79, 86],
        op: |x| x + 5,
        test: |x| if x % 17 == 0 { 5 } else { 0 },
        divisible: 17
    });

    let mut item_map: HashMap<usize, Vec<i64>> = HashMap::new();
    let mut inspection_count: HashMap<usize, i64> = HashMap::new();
    let lcm = monkeys.iter().fold(1, |acc, x| acc * x.divisible);
    for (i, m) in monkeys.iter().enumerate() {
        item_map.insert(i, m.items.iter().map(|x| *x).collect::<Vec<i64>>());
        inspection_count.insert(i, 0);
    }

    let range = if is_part_1 { 0..20 } else { 0..10000 };

    for _ in range {
        for (mi, m) in monkeys.iter().enumerate() {
            let mut monkey_destinations: HashMap<usize, Vec<i64>> = HashMap::new();
            {
                let items = item_map.get_mut(&mi).unwrap();
                for i in items {
                    let cur_count = inspection_count.get_mut(&mi).unwrap();
                    *cur_count += 1;
                    let wl = if is_part_1 { (m.op)(*i) / 3 } else { (m.op)(*i) % lcm };
                    let monkey_index: usize = (m.test)(wl) as usize;
                    if monkey_destinations.contains_key(&monkey_index) {
                        monkey_destinations.get_mut(&monkey_index).unwrap().push(wl);
                    } else {
                        monkey_destinations.insert(monkey_index, vec![wl]);
                    }
                }
                item_map.insert(mi, Vec::new());
            }
            for (mdi, md_items) in monkey_destinations {
                let items = item_map.get_mut(&mdi).unwrap();
                for md_item in md_items {
                    items.push(md_item);
                }
            }
        }
    }
    let mut counts = inspection_count.values().map(|x| *x).collect::<Vec<i64>>();
    counts.sort();
    println!("{:?}", counts.iter().rev().take(2).fold(1, |acc, x| acc * x));
}

fn main() {
    parts(true);
    parts(false);
}