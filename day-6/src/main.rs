#![allow(unused)]
use itertools::Itertools;
fn main() {
    let filepath = r"src/input.txt";

    let input = std::fs::read_to_string(filepath).unwrap().lines().map(String::from).collect::<String>();

    let mut place = usize::max_value();
    let mut str = String::from("");

    // qgds -- 1209
    for (i, c) in input.chars().enumerate() {
        str.replace_last(c, 14);
        if !str.duplicates() && i < place && i > 4{
            place = i + 1;
            println!("Str: {} -- I = {}", str, i);
        }
    }
    println!("Place: {}", place);
}

trait StringLib {
    fn replace_last(&mut self, c: char, len: usize);
    fn duplicates(&self) -> bool;
}

impl StringLib for String {
    fn replace_last(&mut self, c: char, len: usize) {
        if self.len() == len {
            self.remove(0);
            self.push(c);
        } else {
            self.push(c);
        }
    }

    fn duplicates(&self) -> bool {
        for (x, i) in self.chars().enumerate() {
            for (y, c) in self.chars().enumerate() {
                if (i == c) && (y != x)  {
                    return true;
                }
            }
        }
        false
    }
}