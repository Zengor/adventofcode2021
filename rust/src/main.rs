use aoc2021::*;

fn main() {
    println!("Hello, world!");
    let p1 = day2::part1;
    let p2 = day2::part2;
    let input = include_str!("../../input/02-1.txt");
    println!("{}", p1(input).to_string());
    println!("{}", p2(input).to_string());
}
