use std::fs;

mod part_1;
mod part_2;

pub fn run() {
    let input = fs::read_to_string("src/day_1/inputs/actual").expect("Actual input should exist");

    println!("Day 1:");

    println!("  Part 1: {}", part_1::sol(&input));
    println!("  Part 2: {}", part_2::sol(&input));
}
