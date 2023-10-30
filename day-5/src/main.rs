// use std::time::Instant;

use std::{fmt::Display, time::Instant};

fn main() {
    let filepath = r"src/input.txt";
    let input = std::fs::read_to_string(filepath).unwrap().lines().map(String::from).collect::<Vec<String>>();
    
    let start = Instant::now();

    let mut ship = Ship::load_ship(&input);

    for (x, line) in input.into_iter().enumerate() {
        if x > 9 {
            let str = line.split(" ").collect::<Vec<&str>>();
            // println!("{:?}", str);
            let num = str[1].parse::<i32>().unwrap();
            let from = str[3].parse::<usize>().unwrap() - 1;
            let to = str[5].parse::<usize>().unwrap() - 1;
            ship.move_num(num, from, to);
        }
    }

    let duration = start.elapsed();
    println!("{ship}");
    println!("In: {:?}", duration);
}

struct Ship {
    cargo: [Vec<char>; 9]
} impl Ship {
    fn load_ship(input: &Vec<String>) -> Ship {
        let mut ship: [Vec<char>; 9] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (x, line) in input.into_iter().enumerate() {
            if x < 9 {
                let parts = line.chars().collect::<Vec<char>>();
                let mut in_char = false;
                for (x, car) in parts.into_iter().enumerate() {
                    if in_char {
                        let index = ((x + 1) as f32/3.7).round() as usize;
                        ship[index-1].insert(0,car);
                        in_char = false;
                    }

                    if car == '[' {
                        in_char = true;
                    }
                }
            }
        }
        Ship {cargo: ship}
    }

    // Part 1
    fn move_from(&mut self, num: i32, from: usize, to: usize) {
        for _i in 0..num {
            // println!("{i}");
            let crates = self.cargo[from].pop().unwrap();
            // match crates {
            //     Some(x) => self.cargo[to-1].push(x),
            //     None => println!("None"),
            // }
            self.cargo[to].push(crates);
        }
    }

    // Part 2
    fn move_num(&mut self, num: i32, from: usize, to: usize) {
        let mut mov: Vec<char> = Vec::new();
        for _i in 0..num {
            // println!("{i}");
            let crates = self.cargo[from].pop().unwrap();
            // match crates {
            //     Some(x) => self.cargo[to-1].push(x),
            //     None => println!("None"),
            // }
            mov.push(crates);
        }
        mov.reverse();
        self.cargo[to].append(&mut mov);
    }
} impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("{\n");
        for item in &self.cargo {
            string += "(";
            for char in item {
                string.push(*char);
                string.push(',');
                string.push(' ');
            }
            string += ")\n";
        }
        string += "}";
        return write!(f, "{}", string);
    }
}