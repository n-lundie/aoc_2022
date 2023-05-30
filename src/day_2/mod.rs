use std::fs;

mod part_1;
mod part_2;

pub fn run() {
    let input = fs::read("src/day_2/inputs/actual").expect("Actual input should exist");

    println!("Day 2:");

    println!("  Part 1: {}", part_1::sol(&input));
    // println!("  Part 2: {}", part_2::sol(&input));
}
