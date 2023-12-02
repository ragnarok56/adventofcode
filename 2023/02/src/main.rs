use std::fs;
use std::collections::HashMap;

fn main() {
    // what is this rust language.  lets relearn it.   again.   for the third time.  and do it poorly.
    let mut valid_game: HashMap<&str, i32> = HashMap::new();
    valid_game.insert("red", 12);
    valid_game.insert("green", 13);
    valid_game.insert("blue", 14);

    let input = fs::read_to_string("in").unwrap();
    let max_draws = input.lines()
        .map(|x| {
            let draws = x.split(":").skip(1).take(1).collect::<Vec<&str>>()
                .first().unwrap().split(";").collect::<Vec<&str>>();

           let result = draws
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<&str, i32>, d| {
                    d.split(",")
                        .for_each(|c| {
                            let mut citer = c.split_whitespace();
                            let mut t = (citer.next().unwrap().parse::<i32>().unwrap(), citer.next().unwrap());
                            let max = acc.entry(t.1).or_insert(t.0);
                            if max < &mut t.0 {
                                acc.insert(t.1, t.0);
                            }
                        });
                    acc
                });

            result
        })
        .collect::<Vec<HashMap<&str, i32>>>();

    let p1: usize = max_draws
        .iter()
        .enumerate()
        .filter(|x| {
            x.1.iter().all(|d| valid_game.get(d.0).unwrap() >= d.1)
        })
        .map(|x| x.0 + 1)
        .sum();

    let p2: i32 = max_draws 
        .iter()
        .map(|x| x.values().fold(1, |mut acc, v| {
            acc = acc * v;
            acc
        }))
        .sum();
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}
