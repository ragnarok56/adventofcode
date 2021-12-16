extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::collections::HashMap;

fn main() {
    // it is really hard to just parse some args...
    let matches = App::new("adventofcode/2021/14")
        .arg(Arg::with_name("filename")
            .short("f")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("steps")
            .short("s")
            .default_value("1"))
        .get_matches();
    let filename = matches.value_of("filename").unwrap();
    let steps: i32 = matches.value_of("steps").unwrap().parse().unwrap();

    let input = fs::read_to_string(filename).unwrap();

    let mut template = Vec::new();
    let mut insertions = HashMap::new();
    let mut polymer_map = HashMap::new();
    let mut inserted_map: HashMap<String, i64> = HashMap::new();

    // yup, parsing strings is like, super hard
    for (index, line) in input.lines().enumerate() {
        if index == 0 {
            template = line.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x);
                *inserted_map.entry(x.to_string()).or_insert(0) += 1;
                return acc;
            });
            for w in template.windows(2) {
                polymer_map.entry(format!("{}{}", w[0], w[1])).or_insert(1);
            }

        } else if index == 1 {
            continue;
        } else {
            let mut split = line.split(" -> ");
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            insertions.entry(key.to_string()).or_insert(value);
        }
    }

    // let mut polymer = template.clone();
    println!("starting polymer_map:  {:?}", polymer_map);
    println!("starting inserted_map: {:?}", inserted_map);
    println!("--------------------------------");
    for i in 0..steps {
        // let mut polymer_copy = polymer.clone();
        // let mut cur = 0;
        // let mut inserts = 0;
        let mut polymer_map_copy = polymer_map.clone();
        let all_keys = polymer_map.clone().into_keys().collect::<Vec<_>>();
        for key in all_keys {
            if insertions.contains_key(&key) && polymer_map.get(&key).unwrap() > &0 {
                match insertions.get(&*key) {
                    Some(&v) => {
                        println!("found key: {:?}", key);
                        let inserted_char = v.chars().next().unwrap();
                        let mut key_chars = key.chars();
                        let insert_key = key.clone();
                        let num_present = polymer_map.get(&insert_key).unwrap();
                        println!("updating {} in inserted_map with {:?} copies from {}", v, num_present, insert_key);
                        *inserted_map.entry(v.to_string()).or_insert(0) += num_present;
                        *polymer_map_copy.entry(key.clone()).or_insert(0) -= num_present;

                        let key1 = format!("{}{}", key_chars.next().unwrap(), inserted_char);
                        *polymer_map_copy.entry(key1).or_insert(0) += num_present;

                        let key2 = format!("{}{}", inserted_char, key_chars.next().unwrap());
                        *polymer_map_copy.entry(key2).or_insert(0) += num_present;
                        println!("inserted_map: {:?}", inserted_map);
                        println!("polymer_map:  {:?}", polymer_map_copy);
                        println!("--------------------------------")
                    },
                    None => ()
                }
                // inserts += 1;
            }
        }
        polymer_map = polymer_map_copy;
        println!("step {}, {:?}", i + 1, polymer_map);
        // while cur < polymer.len() - 1 {
        //     let key = format!("{}{}", polymer[cur],polymer[cur + 1]);
        //     if insertions.contains_key(&*key) {
        //         match insertions.get(&*key) {
        //             Some(&v) => {
        //                 let inserted_char = v.chars().next().unwrap();
        //                 polymer_copy.insert(cur + inserts + 1, inserted_char);
        //                 let key1 = format!("{}{}", polymer[cur], inserted_char);
        //                 let key2 = format!("{}{}", inserted_char, polymer[cur + 1]);
        //                 let insert_key = key.clone();
        //                 *polymer_map.entry(key).or_insert(0) -= 1;
        //                 *inserted_map.entry(v.to_string()).or_insert(0) += 1;//polymer_map.get(&insert_key).unwrap();
        //                 *polymer_map.entry(key1).or_insert(0) += 1;
        //                 *polymer_map.entry(key2).or_insert(0) += 1;
        //             },
        //             None => ()
        //         }
        //         inserts += 1;
        //     }
        //     cur += 1;
        // }
        // polymer = polymer_copy.clone();
    }

    println!("{:?}", inserted_map);
    println!("{:?}", polymer_map);

    // let mut unique_count_map = polymer
    //     .iter()
    //     .fold(HashMap::new(), |mut acc, x| {
    //         *acc.entry(x).or_insert(0) += 1;
    //         acc
    //     });
    // println!("{:?}", unique_count_map);
    let mut unique_count_vec = inserted_map
        .iter()
        .collect::<Vec<_>>();
    unique_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", unique_count_vec[0].1 - unique_count_vec[unique_count_vec.len() - 1].1);
}