use std::fs;
use std::time::Instant;


pub fn try_one() {
    let start = Instant::now();
    let filename = r"src\input.txt";

    // let mut opp = Vec::new();
    let mut mov = Vec::new();
    let mut tot = 0;
    for line in fs::read_to_string(filename).unwrap().lines() {
        let moves = line.split_at(2);
        // opp.push(String::from(moves.0));
        mov.push((moves.0.trim().to_owned(), moves.1.to_owned()));
    }

    for i in mov.iter() {
        let tup = (i.0.as_str(), i.1.as_str());
        let start = match tup.1 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("LINE 20"),
        };

        // A is Rock
        // B is Paper
        // C is Scissors
        // -------------------
        // X is Rock
        // Y is Paper
        // Z is Scissors
        let rest = match tup {
            ("A", "X") => 3,
            ("B", "X") => 0,
            ("C", "X") => 6,
            ("A", "Y") => 6,
            ("B", "Y") => 3,
            ("C", "Y") => 0,
            ("A", "Z") => 0,
            ("B", "Z") => 6,
            ("C", "Z") => 3,
            _ => panic!("LINE 42: {}, {}",tup.0, tup.1),
        };
        tot += start + rest;
    }
    let duration = start.elapsed();
    println!("try_one-- Total: {} \nTook {:?}", tot, duration);
}
