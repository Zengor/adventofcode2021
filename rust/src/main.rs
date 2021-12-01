use aoc2021::*;

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        day1::part1(include_str!("../../input/01-1.txt")).to_string()
    );
    println!(
        "{}",
        day1::part2(include_str!("../../input/01-1.txt")).to_string()
    );
    println!(
        "{}",
        day1::part2_noitertools(include_str!("../../input/01-1.txt")).to_string()
    );
}
