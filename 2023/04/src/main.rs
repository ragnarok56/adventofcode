use std::fs;
use std::collections::HashSet;

fn main() {
    
    let input = fs::read_to_string("in").unwrap();
    let p1 = input.lines()
        .map(|x| {
            let mut all_nums = x.split(&[':', '|'][..]).skip(1);
            let winning: HashSet<u32> = HashSet::from_iter(all_nums.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            let played: HashSet<u32> = HashSet::from_iter(all_nums.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            let num_wins = winning.intersection(&played).count();
            let mut score = 0;
            if num_wins > 0 {
                score = 2i32.pow((num_wins - 1).try_into().unwrap())
            }
            score
        })
        .sum::<i32>();
    
    println!("p1: {:?}", p1);
}