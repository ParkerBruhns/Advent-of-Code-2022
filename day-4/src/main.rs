use std::{ops::Range, time::Instant};

fn main() {
    let file_path = r"src/input.txt";
    let input = std::fs::read_to_string(file_path).unwrap().lines().map(String::from).collect::<Vec<String>>();

    let start = Instant::now();

    let mut total = 0;
    for line in input.into_iter() {
        let mut parts = line.split(',');
        let (pair_1, pair_2) = (parts.next().unwrap(), parts.next().unwrap());
        if range_overlap(to_range(pair_1), to_range(pair_2)) {
            total += 1;
        }
    }

    let duration = start.elapsed();

    println!("Total: {} in {:?}", total, duration);

}

fn to_range(str: &str) -> Range<i32>{
    let parts = str.split('-').collect::<Vec<&str>>();
    let val = parts.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    val[0]..val[1]
}

//part 1
fn range_consume(range_1: Range<i32>, range_2: Range<i32>) -> bool {
    if range_1.start <= range_2.start && range_1.end >= range_2.end {
        return true;
    } else if range_2.start <= range_1.start && range_2.end >= range_1.end {
        return true;
    }
    false
}

//part 2
fn range_overlap(range_1: Range<i32>, range_2: Range<i32>) -> bool {
    if range_1.end >= range_2.start && range_1.start <= range_2.end {
        return true;
    } else if range_2.end >= range_1.start && range_2.start <= range_1.end {
        return true;
    }
    false
}

