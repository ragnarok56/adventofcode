use std::fs;
use std::collections::{HashMap,HashSet};
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("in")
        .unwrap();

    let mut page_orders: HashMap<i32, HashSet<i32>> = HashMap::new();

    input
        .lines()
        .take_while(|x| x.contains('|'))
        .for_each(|x| {
            let mut pages = x.splitn(2, '|');
            let l: i32 = pages.next().unwrap().parse().unwrap();
            let r: i32 = pages.next().unwrap().parse().unwrap();
            page_orders.entry(l).or_default().insert(r);
        });

    let updates = input
        .lines()
        .filter(|x| x.contains(','))
        .map(|x| {
            x
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum_correct = 0;
    let mut sum_incorrect = 0;

    for u in updates {
        let mut correct = true;
        let mut seen: HashSet<i32> = HashSet::new();
        for p in u.iter() {

            let cur = page_orders.get(p);
            if cur.is_some() && cur.unwrap().iter().any(|x| seen.contains(x)) {
                correct = false;
            }
            seen.insert(*p);
        }

        if correct {
            sum_correct += u.get(u.len() / 2).unwrap();
        } else {
            let mut update_vec: Vec<i32> = seen.into_iter().collect::<Vec<_>>();
            update_vec.sort_by(|a, b| {
                let a_item: Option<&HashSet<i32>> = page_orders.get(a);
                if a_item.is_some() && a_item.unwrap().contains(b) {
                    return Ordering::Less
                }
                let b_item: Option<&HashSet<i32>> = page_orders.get(b);
                if b_item.is_some() && b_item.unwrap().contains(a) {
                    return Ordering::Greater
                }

                Ordering::Equal
            });
            sum_incorrect += update_vec.get(update_vec.len() / 2).unwrap();
        }
    }
    println!("{:?}", sum_correct);
    println!("{:?}", sum_incorrect);
}
