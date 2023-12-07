use std::fs;

#[derive(Debug)]
struct RangeMap {
    dest_start: u64,
    src_start: u64,
    length: u64
}

impl RangeMap {
    fn get(&self, src: u64) -> Option<u64> {
        if self.src_start <= src && self.src_start + self.length >= src {
            let difference = src - self.src_start;
            return Some(self.dest_start + difference)
        }
        None
    }
}

fn get_location(mut src: u64, map: &Vec<Vec<RangeMap>>) -> u64 {
    for m in map.iter() {
        for r in m {
            let next = r.get(src);
            if next.is_some() {
                src = next.unwrap();
                break;
            }
        }
    }
    src
}

fn create_range(values: &str) -> RangeMap {
    let mut split_vals = values.split_whitespace();
    RangeMap {
        dest_start: split_vals.next().unwrap().parse::<u64>().unwrap(),
        src_start: split_vals.next().unwrap().parse::<u64>().unwrap(),
        length: split_vals.next().unwrap().parse::<u64>().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("in").unwrap();

    let mut input_lines = input.lines();

    let seed_values = input_lines.next().unwrap().split(':').skip(1).next().unwrap().split_whitespace().collect::<Vec<_>>();
    input_lines.next();

    let seed_soil = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let soil_fert = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let fert_water = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let water_light = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let light_temp = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let temp_humid = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();
    let humid_location = input_lines.by_ref().take_while(|x| !x.is_empty()).skip(1).map(|x| create_range(x)).collect::<Vec<RangeMap>>();

    let map_transitions: Vec<Vec<RangeMap>> = vec![seed_soil, soil_fert, fert_water, water_light, light_temp, temp_humid, humid_location];

    let seeds = seed_values.iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let p1 = seeds
        .iter()
        .map(|s| get_location(*s, &map_transitions))
        .min()
        .unwrap();

    println!("p1: {:?}", p1);

    let p2 = seed_values
        .chunks(2)
        .map(|x| {
            let start = x.get(0).unwrap().parse::<u64>().unwrap();
            let length = x.get(1).unwrap().parse::<u64>().unwrap();
            let range = start..start+length+1;
            
            println!("processing batch of seeds: {:?}-{:?}", start, length);

            return range
                .map(|s| get_location(s, &map_transitions))
                .min()
                .unwrap();
        })
        .min()
        .unwrap();

    println!("p2: {:?}", p2); 
}
