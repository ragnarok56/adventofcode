use std::fs;

fn part1() -> i32 {
    fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|line: &str| {
            let matches = line
                .split(' ')
                .map(|x| {
                    match x {
                        "A" | "X" => 1,
                        "B" | "Y" => 2,
                        "C" | "Z" => 3,
                        _ => 0
                    }
                })
                .collect::<Vec<_>>();

            let my_play = matches.get(1).unwrap();
            let match_result = matches.get(0).unwrap() - my_play;
            let score = match match_result {
                -1 => 6,
                -2 => 0,
                1 => 0,
                2 => 6,
                _ => 3
            };
            return score + my_play;
        })
        .sum::<i32>()
}

fn part2() -> i32{
    fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|line: &str| {
            let matches = line
                .split(' ')
                .collect::<Vec<_>>();

            let opp_play = match *matches.get(0).unwrap() {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0
            };

            let my_play = match *matches.get(1).unwrap() {
                "X" => match opp_play {
                    1 => 3,
                    2 => 1,
                    3 => 2,
                    _ => 0
                },
                "Y" => opp_play + 3,
                "Z" => match opp_play {
                    1 => 8,
                    2 => 9,
                    3 => 7,
                    _ => 0
                },
                _ => 0
            };
            return my_play;
        })
        .sum::<i32>()
}

fn main() {
    let result = part1();
    println!("{:?}", result);
    let result = part2();
    println!("{:?}", result);
}