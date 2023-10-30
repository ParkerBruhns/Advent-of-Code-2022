use std::fs;
use std::time::Instant;

pub fn part_1() {
    let filename = r"src\input.txt";

    let input = fs::read_to_string(filename).unwrap().lines().map(String::from).collect::<Vec<String>>();

    let start = Instant::now();

    let mut total = 0;
    for line in input {
        let leng = line.chars().count() as f32 / 2.0;
        let (first, second) = line.split_at(leng as usize);
        let same = same_char(first, second);
        total += match same {
            Some(v) => v.value(),
            None => 0,
        };

        // println!("Leng: {leng}\nFirst: {first}\nSecond: {second}");
    }

    let duration = start.elapsed();

    println!("Prev: 8176");
    println!("Total: {} in {:?}", total, duration);
}

pub fn same_char(first: &str, second: &str) -> Option<char> {
    let first_vec = first.chars().collect::<Vec<char>>();
    let second_vec = second.chars().collect::<Vec<char>>();

    for first_char in first_vec.iter() {
        for second_char in second_vec.iter() {
            if first_char == second_char {
                return Some(first_char.to_owned());
            }
        }
    }
    None
}

trait Priority {
    fn value(self) -> u32;
}

impl Priority for char {
    fn value(self) -> u32 {
        match self {
            'a'..='z' => self as u32 - 'a' as u32 + 1,
            'A'..='Z' => self as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid Character"),
        }
    }
}
