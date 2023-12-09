use std::{fs, collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32
}

impl Hand {
    fn score(&self, jacks_wild: bool) -> u32 {
        let map = self.cards
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                if !(jacks_wild && *x == 'J') {
                    *acc.entry(x).or_insert(0) += 1;
                }
                acc
            });
        // edge case for when its all wildcards
        if jacks_wild && map.len() == 0 {
            return 7
        }
        let num_wild = if jacks_wild { self.cards.iter().filter(|x| **x == 'J').count() } else { 0 };
        let most_of = map.values().max().unwrap();
        // i hate what this has become
        // basically just count # of uniq cards in the hand and come up with what we think that hand is
        // for duplicate counts, delve deeper and compare count of card that we have most of to determine
        // what possible hand it could be
        // wildcards make this messy and have to fill in gaps, usually by just bumping up score
        // based on how many wildcards they had. 
        // example: AJAJ8 is 2 uniques when J is excluded
        //   most of is A (2), which falls through again to
        //   compare 2 with what the max _could_ have been (4) if we exclude wildcards,
        //   find out its the same and therefore best is a 4 of a kind, so its 6.  whew.
        let score = match map.len() {
            // 5 of a kind always
            1 => 7, 
            // 2 uniq cards, worst case is 3 wildcards, which would mean 4 of a kind is best hand
            2 => {
                match most_of {
                    4 => 6,
                    3 => 5 + num_wild,
                    _ => {
                        match *most_of == 4 - num_wild {
                            true => 6,
                            false => 5
                        }
                    }
                }
            },
            // 3 uniq cards, worst case is 2 wildcards, which would mean 3 of a kind is best hand
            3 => {
                match most_of {
                    3 => 4,
                    2 => 3 + num_wild, 
                    _ => {
                        match *most_of == 3 - num_wild {
                            true => 4,
                            false => 3
                        }
                    }
                }
            }
            // if there are 4 uniq cards, it _has_ to be one pair, so dont include wildcards
            4 => 2,
            // any wildcard bumps high card to one pair
            5 => 1 + num_wild,
            _ => panic!()
        };
        score.try_into().unwrap()
    }

    fn cmp(&self, other: &Hand, jacks_wild: bool) -> Ordering {
        let score = self.score(jacks_wild);
        let other_score: u32 = other.score(jacks_wild);
        if score == other_score {
            for i in 0..self.cards.len() {
                let card_value = self.cards.get(i).unwrap().camel_card_value(jacks_wild);
                let other_card_value = other.cards.get(i).unwrap().camel_card_value(jacks_wild);
                if card_value == other_card_value {
                    continue
                }
                return card_value.cmp(&other_card_value)
            }
        }
        score.cmp(&other_score)
    }
}

trait CamelCardChar {
    fn camel_card_value(&self, jacks_wild: bool) -> u32;
}

impl CamelCardChar for char {
    fn camel_card_value(&self, jacks_wild: bool) -> u32 {
        match self {
            'A' => 14,
            'T' => 10,
            'J' => match jacks_wild {
                true => 0,
                false => 11
            },
            'Q' => 12,
            'K' => 13,
            _ => self.to_digit(10).unwrap()
        }
    }
}

fn calculate_winnings(hands: &Vec<Hand>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(i, x)| -> u32 {
            let try_into: u32 = i.try_into().unwrap();
            let score = (try_into + 1) * x.bid;
            score
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("in").unwrap();
    let lines = input.lines();

    let mut hands = lines.map(|x| {
        let mut split = x.split_whitespace();
        let hand = split.next();
        let bid = split.next();
        Hand {
            cards: hand.unwrap().chars().collect::<Vec<_>>(),
            bid: bid.unwrap().parse::<u32>().unwrap()
        }
    }).collect::<Vec<_>>();

    hands.sort_by(|a, b| a.cmp(b, false));
    
    let p1: u32 = calculate_winnings(&hands);
    println!("p1: {:?}", p1);

    hands.sort_by(|a, b| a.cmp(b, true));

    let p2: u32 = calculate_winnings(&hands);
    println!("p2: {:?}", p2);
}