use std::fs;

fn get_wins(time: i64, distance: i64) -> i64 {
    let mut wins = 0;
    let mut max = time / 2;
    let mut rem = time - max;
    let diff = if max - rem == 0 { 1 } else { 0 };
    if max * rem > distance {
        wins += 1;
        max -= 1;
        rem += 1;
        while max * rem > distance {
            wins +=1;
            max -= 1;
            rem += 1;
        }
    }
    wins * 2 - diff
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let mut lines = input.lines();

    let times = lines.by_ref().next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut distance = lines.by_ref().next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let races = times.iter().zip(distance.iter_mut());

    let p1 = races.fold(1, |mut acc, r| {
        let wins = get_wins(*r.0, *r.1);
        if wins > 0 {
            acc *= wins;
        }
        acc
    });
    println!("p1: {:?}", p1);

    let times_2 = times.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<i64>().unwrap();
    let distance_2 = distance.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<i64>().unwrap();

    let p2 = get_wins(times_2, distance_2);

    println!("p2: {:?}", p2);
}
