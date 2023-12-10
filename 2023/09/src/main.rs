use std::fs;

#[derive(Debug)]
struct Report {
    sequences: Vec<Vec<i64>>
}

impl Report {
    fn predict_end(&self) -> i64 {
        let mut seq_copy = self.sequences.clone();
        seq_copy.reverse();
        let prediction = seq_copy
            .iter()
            .fold(0,|acc: i64, x| {
                let seq_prediction = acc + x.last().unwrap();
                seq_prediction
            });
        prediction
    }  
    fn predict_start(&self) -> i64 {
        let mut seq_copy = self.sequences.clone();
        seq_copy.reverse();
        let prediction = seq_copy
            .iter()
            .fold(0,|acc: i64, x| {
                let seq_prediction = x.first().unwrap() - acc;
                seq_prediction
            });
        prediction
    }  
}

fn generate_sequences(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut finished_sequences = false;
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    sequences.push(history.to_vec());
    let mut cur_sequence = history.clone();

    while !finished_sequences {
        let sequence = cur_sequence.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        if sequence.iter().all(|x| *x == 0i64) {
            finished_sequences = true;
        }
        sequences.push(sequence.clone());
        cur_sequence = sequence;
    }
    sequences
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();

    let reports = lines
        .map(|x| {
            let history = x.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let sequences = generate_sequences(&history);
            Report { sequences }
        })
        .collect::<Vec<_>>();

    let p1: i64 = reports.iter().map(|x| x.predict_end()).sum();
    println!("p1: {:?}", p1);
    
    let p2: i64 = reports.iter().map(|x| x.predict_start()).sum();
    println!("p2: {:?}", p2);
}