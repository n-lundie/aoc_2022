use std::fs;

mod part_1;

pub fn run() {
    let input = fs::read_to_string("src/day_2/inputs/actual").expect("Actual input should exist");

    println!("Day 2:");

    println!("  Part 1: {}", part_1::sol(&input));
}
