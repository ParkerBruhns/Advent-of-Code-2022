//Answer: 70374

#![allow(unused)]

use std::env;
use std::fs;

fn main() {

    let filepath = r"src/input.txt";
    println!("In file: {filepath}");

    let mut result: Vec<String> = fs::read_to_string(filepath).unwrap().lines().map(String::from).collect();
    let mut elves = group_to_elves(result);

    // Max cals
    // let sum = elves.into_iter().map(|x| x.iter().sum::<i32>()).max().unwrap();
    // Top 3 cals
    let mut result = elves.into_iter().map(|x| x.iter().sum::<i32>()).collect::<Vec<i32>>();
    result.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let sum: i32 = result.into_iter().rev().take(3).sum();

    println!("Most cals: {sum}");
}

fn group_to_elves(group: Vec<String>) -> Vec<Vec<i32>> {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in group.into_iter() {
        if (line != "") {
            elf.push(line.parse::<i32>().unwrap());
        } else {
            elves.push(elf.clone());
            elf.clear();
        }
    }

    elves
}
