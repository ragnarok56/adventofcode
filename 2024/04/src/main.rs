use std::fs;

fn load_input() -> Vec<Vec<char>> {
    fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|x| {
            x.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x);
                acc
            })
        })
        .collect()
}


fn find_xmas(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> i32 {
    let height = matrix.len() as i32 - 1;
    let length = matrix.first().unwrap().len() as i32 - 1;
    let directions: [(i32, i32); 8] = [(0, 1),(0, -1),(1, 0),(1, 1),(1,-1),(-1,0),(-1, -1),(-1,1)];
    let move_match = ['X', 'M', 'A', 'S'];
    let mut num_matched = 0;
    for d in directions {
        let mut next_pos = (i, j);
        let mut valid = true;
        for letter in move_match.iter().enumerate() {
            let cur_pos_letter = matrix.get(next_pos.0 as usize).unwrap().get(next_pos.1 as usize).unwrap();
            if cur_pos_letter != letter.1 {
                valid = false;
                break;
            }
            if letter.0 == 3 { break; }
            next_pos = (next_pos.0 + d.0, next_pos.1 + d.1);
            if next_pos.0 < 0 || next_pos.0 > length || next_pos.1 < 0 || next_pos.1 > height {
                valid = false;
                break;
            }
        }
        if valid { num_matched += 1; }
    }
    num_matched
}


fn find_x_mas(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> i32 {
    let height = matrix.len() as i32 - 1;
    let length = matrix.first().unwrap().len() as i32 - 1;
    let pairs: Vec<Vec<(i32, i32)>> = vec![
        vec![(-1, -1),(1, 1)],
        vec![(-1, 1),(1, -1)]
    ];
    let matched: i32 = pairs.iter()
        .map(|pair| {
            let pair_chars = pair.iter().map(|dir| {
                let next_pos = (i + dir.0, j + dir.1);
                if next_pos.0 < 0 || next_pos.0 > length || next_pos.1 < 0 || next_pos.1 > height {
                    return Option::None
                }
                matrix.get(next_pos.0 as usize).unwrap().get(next_pos.1 as usize)
            })
            .collect::<Vec<Option<&char>>>();
            if pair_chars.iter().any(|p| p.is_none()) {
                return 0
            }
            let mut chars = pair_chars.iter().map(|p| p.unwrap().to_string()).collect::<Vec<String>>();
            chars.sort();
            if chars.join("") == "MS" { 1 } else { 0 }
        })
        .sum();
    if matched == 2 { 1 } else { 0 }
}

fn main() {
    let input = load_input();
    let mut p1_total = 0;
    let mut p2_total = 0;
    for (i, r) in input.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'X' {
                p1_total += find_xmas(i as i32, j as i32, &input);
            }
            if *c == 'A' {
                p2_total += find_x_mas(i as i32, j as i32, &input);
            }
        }
    }

    println!("{:?}", p1_total);
    println!("{:?}", p2_total);
}
