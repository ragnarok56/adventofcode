use std::fs;

fn check_levels(report: &Vec<i32>) -> bool {
    let mut increasing: Option<bool> = Option::None;
    report
        .windows(2)
        .all(|p| {
            let mut level_safe = true;

            let diff = p[0] - p[1];
            if increasing.is_none() {
                increasing = if diff > 0 { Some(true) } else { Some(false) }
            } else {
                if increasing.unwrap() && diff < 0 {
                    level_safe = false
                }
                if !increasing.unwrap() && diff > 0 {
                    level_safe = false
                }
            }
            level_safe && diff.abs() > 0 && diff.abs() <= 3
        })
}

fn main() {
    let input = fs::read_to_string("in").unwrap();

    let reports = input
        .lines()
        .map(|x| {
            x.split(' ').collect::<Vec<&str>>()
                .iter()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let p1: i32 = reports
        .iter()
        .map(|x| if check_levels(x) { 1 } else { 0 })
        .sum();

    println!("{:?}", p1);

    let p1: i32 = reports
        .iter()
        .map(|x| {
            let mut is_safe = check_levels(&x);

            if !is_safe {
                let num_levels = x.len();
                // brute force by removing a level and checking for any that arent already good
                for i in 0..num_levels {
                    // goofy slicing
                    is_safe = check_levels(&[&x[0..i], &x[i+1..]].concat());
                    if is_safe {
                        break;
                    }
                }
            }
            if is_safe { 1 } else { 0 }
        }).sum();

    println!("{:?}", p1);
}
