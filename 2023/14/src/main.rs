use std::fs;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn roll_stones_north(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let matrix_transposed = transpose(&matrix);
    let mut new_matrix: Vec<Vec<char>> = Vec::new();
    for m in matrix_transposed.iter() {
        let split = m.split(|x| *x == '#').collect::<Vec<_>>();
        let sorted = split.iter().map(|x| {
            let mut copy = x.to_vec();
            copy.sort();
            copy.reverse();
            copy
        })
        .collect::<Vec<_>>();
        let back_together = sorted.join(&'#');
        new_matrix.push(back_together);
    }
    transpose(&new_matrix)
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

    let matrix = roll_stones_north(&matrix);

    let height = matrix.len();
    let p1 = matrix
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let distance = height - i;
            let load: usize = x.iter().filter(|c| **c == 'O').count() * distance;
            load
        })
        .sum::<usize>();
    println!("p1: {:?}", p1);
    
}