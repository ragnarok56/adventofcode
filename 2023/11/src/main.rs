use std::fs;
use itertools::Itertools;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn measure_universe(matrix: &Vec<Vec<char>>, factor: usize) -> i64 {
    let rows_to_expand = matrix.iter()
        .enumerate()
        .filter(|x| x.1.iter().all(|c| *c == '.'))
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let cols_to_expand = transpose(&matrix).iter()
        .enumerate()
        .filter(|x| x.1.iter().all(|c| *c == '.'))
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let factor_offset = factor - 1;
    for (row, _) in matrix.iter().enumerate() {
        let row_exp = rows_to_expand.iter().filter(|x| **x <= row).count();
        for (col, c) in matrix[row].iter().enumerate() {
            let col_exp = cols_to_expand.iter().filter(|x| **x <= col).count();
            if *c == '#' {
                galaxies.push((row + row_exp * factor_offset, col + col_exp * factor_offset));
            }
        }
    }

    galaxies
        .iter()
        .combinations(2)
        .map(|x| {
            ((x[0].0 as i64) - (x[1].0 as i64)).abs() + ((x[0].1 as i64) - (x[1].1 as i64)).abs()
        })
        .sum::<i64>()
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();
    let matrix: Vec<Vec<char>> = lines
        .map(|x| {
            x.chars().fold(Vec::new(), |mut acc, x| {
                acc.push(x);
                acc
            })
        })
        .collect();

    let p1 = measure_universe(&matrix, 2);
    println!("p1: {:?}", p1);

    let p2 = measure_universe(&matrix, 1_000_000);
    println!("p2: {:?}", p2);
}