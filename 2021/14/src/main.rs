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

    let mut insertions = HashMap::new();
    let mut polymer_map = HashMap::new();
    let mut inserted_map: HashMap<String, i64> = HashMap::new();

    // yup, parsing strings is like, super hard
    for (index, line) in input.lines().enumerate() {
        if index == 0 {
            // take first line template and prefill inserted_map and polymer_map
            let template = line.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x);
                *inserted_map.entry(x.to_string()).or_insert(0) += 1;
                return acc;
            });
            // need as vec to use windows to generate pairs for polymer_map
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

    for _ in 0..steps {
        // must copy map so we can modify the copy and retain original for lookups
        // as we iterate each step
        let mut polymer_map_copy = polymer_map.clone();
        // so many clones
        let all_keys = polymer_map.clone().into_keys().collect::<Vec<_>>();

        for key in all_keys {
            // only process polymer if it "exists" and has a positive number of occurances
            if insertions.contains_key(&key) && polymer_map.get(&key).unwrap() > &0 {
                match insertions.get(&*key) {
                    Some(&v) => {
                        let insert_key = key.clone();
                        // pull out how many times this polymer occured. this needs to be
                        // added to the number of times the insert value has been added
                        // to account for each copy.  similarly, reduce the number of occurances
                        // of the polymer to 0 (since we just split it everywhere)
                        let num_present = polymer_map.get(&insert_key).unwrap();
                        *inserted_map.entry(v.to_string()).or_insert(0) += num_present;
                        *polymer_map_copy.entry(insert_key).or_insert(0) -= num_present;

                        // now generate new copies of polymers using the original pair
                        // so AB -> C  insert would add an AC and CB pair.  Add the same
                        // number of these for however many of the original (AB) were in
                        // the polymer_map
                        let mut key_chars = key.chars();
                        let inserted_char = v.chars().next().unwrap();
                        let key1 = format!("{}{}", key_chars.next().unwrap(), inserted_char);
                        *polymer_map_copy.entry(key1).or_insert(0) += num_present;

                        let key2 = format!("{}{}", inserted_char, key_chars.next().unwrap());
                        *polymer_map_copy.entry(key2).or_insert(0) += num_present;
                    },
                    None => ()
                }
            }
        }
        polymer_map = polymer_map_copy;
    }

    let mut unique_count_vec = inserted_map
        .iter()
        .collect::<Vec<_>>();
    unique_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", unique_count_vec[0].1 - unique_count_vec[unique_count_vec.len() - 1].1);
}