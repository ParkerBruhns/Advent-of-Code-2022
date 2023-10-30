#![allow(unused)]
mod try_one;
mod try_two;

use crate::try_one::try_one;
use crate::try_two::try_two;

use std::fs;
use std::time::Instant;

fn main() {
    try_one();
    try_two();

    let start = Instant::now();

    let filename = r"src\input.txt";
    let input = fs::read_to_string(filename).unwrap().lines().map(String::from).collect::<Vec<String>>();

    let mut total: i32 = 0;
    for line in input {
        println!("{line}");
        total += points(from_str(line));
    }

    let duration = start.elapsed();
    println!("Total: {total}");
    println!("Success in {:?}", duration);
}

pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
} impl Move {
    pub fn val(self) -> i32 {
        self as i32
    }
}

pub enum Outcome {
    Win = 6,
    Tie = 3,
    Loose = 0
} impl Outcome {
    pub fn val(self) -> i32 {
        self as i32
    }
}

pub fn points(moves: (Move, Outcome)) -> i32 {
    use Move::*;
    use Outcome::*;
    match (moves.0, moves.1) {
        (Rock, Win) => Rock.val() + Win.val(),
        (Rock, Tie) => Rock.val() + Tie.val(),
        (Rock, Loose) => Rock.val() + Loose.val(),
        (Paper, Win) => Paper.val() + Win.val(),
        (Paper, Tie) => Paper.val() + Tie.val(),
        (Paper, Loose) => Paper.val() + Loose.val(),
        (Scissors, Win) => Scissors.val() + Win.val(),
        (Scissors, Tie) => Scissors.val() + Tie.val(),
        (Scissors, Loose) => Scissors.val() + Loose.val(),
    }
}

pub fn string_to_pts(input: String) -> i32 {
    let mut parts = input.split(" ");
    let user = parts.next().unwrap();
    let opponent = parts.next().unwrap();

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

pub fn from_str(input: String) -> (Move, Outcome) {
    use Move::*;
    use Outcome::*;
    let mut parts = input.split(" ");
    let end = parts.next().unwrap();
    let opponent = parts.next().unwrap();

    // println!("User: {} ---- Opponent: {}", user, opponent);

    match (end, opponent) {
        ("A", "X") => (Scissors, Loose),
        ("A", "Y") => (Rock, Tie),
        ("A", "Z") => (Paper, Win),
        ("B", "X") => (Rock, Loose),
        ("B", "Y") => (Paper, Tie),
        ("B", "Z") => (Scissors, Win),
        ("C", "X") => (Paper, Loose),
        ("C", "Y") => (Scissors, Tie),
        ("C", "Z") => (Rock, Win),
        (_,_) => (Rock, Win),
    }
}