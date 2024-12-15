use std::fs;
use itertools::Itertools;


fn load_input(path: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| {
            let mut split = x.split(':');
            let total: i64 = split.next().unwrap().parse().unwrap();
            let nums: Vec<i64> = split.next().unwrap()
                .split(' ')
                .filter(|n| n.len() > 0)
                .map(|n: &str| n.parse().unwrap()).collect();
            (total, nums)
        })
        .collect()
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Operator {
    Add,
    Mult,
    Combine
}

fn process(equations: &Vec<(i64, Vec<i64>)>, operators: Vec<Operator>) -> i64 {
    let mut result = 0;
    for (total, nums) in equations {
        let len_nums = nums.len();

        let mut operator_combos = (0..len_nums - 1).map(|_| operators.clone()).multi_cartesian_product();

        let first = *nums.first().unwrap();

        let found = operator_combos.any(|operators| {
            let remaining = nums.iter().skip(1);
            let result = remaining.zip(operators).fold(first, |acc, (num, oper)| {
                match oper {
                    Operator::Add => acc + num,
                    Operator::Mult => acc * num,
                    Operator::Combine => (acc.to_string() + &num.to_string()).parse().unwrap()
                }
            });
            result == *total
        });

        if found {
            result += total;
        }
    }
    result
}

fn main() {
    let equations = load_input("in");

    let p1 = process(&equations, vec![Operator::Add, Operator::Mult]);
    println!("{}", p1);

    let p2 = process(&equations, vec![Operator::Add, Operator::Mult, Operator::Combine]);
    println!("{}", p2);
}