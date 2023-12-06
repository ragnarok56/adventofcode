use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;



fn main() {
    let mut total_scratchers = 0;
    let mut copy_map: HashMap<usize, usize> = HashMap::new();
    let input = fs::read_to_string("in").unwrap();
    let draws: Vec<(usize, i32, usize)> = input.lines().enumerate()
        .map(|(i, x)| {
            // count current scratcher always
            total_scratchers += 1;
            let mut all_nums = x.split(&[':', '|'][..]).skip(1);
            let winning: HashSet<u32> = HashSet::from_iter(all_nums.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            let played: HashSet<u32> = HashSet::from_iter(all_nums.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            let num_wins = winning.intersection(&played).count();
            let mut score = 0;
            if num_wins > 0 {
                score = 2i32.pow((num_wins - 1).try_into().unwrap())
            }
            // load how many copies of the current scratcher we've won
            let making_copies: usize = *copy_map.entry(i).or_insert(0) as usize;
            for r in 0..num_wins {
                total_scratchers += making_copies + 1;
                // add more scratchers for each subsequent draw
                let val: &mut usize = copy_map.entry(r + i + 1).or_insert(0);
                *val += making_copies + 1;
            }
            (i, score, num_wins)
        })
        .collect();

    let p1 = draws.iter().map(|(_, x, _)| x).sum::<i32>();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", total_scratchers);
}