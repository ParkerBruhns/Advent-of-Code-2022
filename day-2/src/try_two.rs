#![allow(unused)]
use std::fs;
use std::time::Instant;


pub fn try_two() {
    let start = Instant::now();

    let filename = r"src\input.txt";
    let input = fs::read_to_string(filename).unwrap().lines().map(String::from).collect::<Vec<String>>();

    let mut total: i32 = 0;
    for line in input {
        // println!("{line}");
        // total += Move::points(Move::from_str(line));
        total += Move::string_to_pts(line);
    }

    let duration = start.elapsed();
    println!("Total: {total}");
    println!("Success in {:?}", duration);
}

pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn points(moves: (Move, Move)) -> i32 {
        match (moves.0, moves.1) {
            (Move::Rock, Move::Rock) => 4,
            (Move::Rock, Move::Paper) => 1,
            (Move::Rock, Move::Scissors) => 7,
            (Move::Paper, Move::Rock) => 8,
            (Move::Paper, Move::Paper) => 5,
            (Move::Paper, Move::Scissors) => 2,
            (Move::Scissors, Move::Rock) => 3,
            (Move::Scissors, Move::Paper) => 9,
            (Move::Scissors, Move::Scissors) => 6,
        }
    }

    pub fn string_to_pts(input: String) -> i32 {
        let mut parts = input.split(" ");
        let opponent = parts.next().unwrap();
        let user = parts.next().unwrap();

        // println!("User: {} ---- Opponent: {}", user, opponent);

        match (user, opponent) {
            ("X", "A") => 4,
            ("X", "B") => 1,
            ("X", "C") => 7,
            ("Y", "A") => 8,
            ("Y", "B") => 5,
            ("Y", "C") => 2,
            ("Z", "A") => 3,
            ("Z", "B") => 9,
            ("Z", "C") => 6,
            (_,_) => 10_000_000,
        }
    }

    pub fn from_str(input: String) -> (Move, Move) {
        let mut parts = input.split(" ");
        let opponent = parts.next().unwrap();
        let user = parts.next().unwrap();

        // println!("User: {} ---- Opponent: {}", user, opponent);

        match (user, opponent) {
            ("X", "A") => (Move::Rock, Move::Rock),
            ("Y", "A") => (Move::Paper, Move::Rock),
            ("Z", "A") => (Move::Scissors, Move::Rock),
            ("X", "B") => (Move::Rock, Move::Paper),
            ("Y", "B") => (Move::Paper, Move::Paper),
            ("Z", "B") => (Move::Scissors, Move::Paper),
            ("X", "C") => (Move::Rock, Move::Scissors),
            ("Y", "C") => (Move::Paper, Move::Scissors),
            ("Z", "C") => (Move::Scissors, Move::Scissors),
            (_,_) => (Move::Rock, Move::Rock),
        }
    }
}