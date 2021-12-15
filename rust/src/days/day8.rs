use bitvec::prelude::*;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split("|").skip(1).next().unwrap())
        .map(|output_part| output_part.split_whitespace())
        .flatten()
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

fn to_bitmap(digit: &str) -> BitArray<Lsb0, u8> {
    let mut out = BitArray::zeroed();
    for c in digit.chars() {
        match c {
            c @ 'a'..='g' => out.set((c as usize) - 97, true),
            _ => panic!("invalid input"),
        }
    }
    out
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
//
// 1 | 2 bits
// 7 | 3 bits
// 4 | 4 bits
//              (x|1) | (x|4) |
// 2 | 5 bits |   6   | **7** |
// 3 | 5 bits | **5** |   6   |
// 5 | 5 bits |   6   |   6   |
// 0 | 6 bits |   6   | **7** |
// 6 | 6 bits | **7** |   7   |
// 9 | 6 bits |   6   | **6** |
//
// 8 | 7 bits
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (mut digits, output_part) = line
            .split(" | ")
            .map(|part| part.split(" ").map(to_bitmap).collect::<Vec<_>>())
            .collect_tuple()
            .unwrap();
        digits.sort_by_key(|b| b.count_ones());
        // once we have them sorted by count, here is what we know for sure:
        // idx -> corresponding digit)
        // [0] -> <ONE> (2 'true' bits)
        // [1] -> <SEVEN> (3 'true' bits)
        // [2] -> <FOUR> (4 'true' bits)
        // [9] -> <EIGHT> (7 'true' bits)
        let one = digits[0];
        let four = digits[2];
        let value = output_part
            .into_iter()
            .map(|out_digit| {
                match out_digit.count_ones() {
                    2 => '1',
                    3 => '7',
                    4 => '4',
                    7 => '8',
                    // Below we figure out digits based on combining
                    // the bitmaps of a known number with the unknown number
                    // and seeing how many 'true' bits they have.
                    // See the larger comment before this function for the table.
                    // Because matches go from top to bottom, we can use the order
                    // to make sure the digit didn't matcha previous condition.
                    // count = 5 must be <TWO>,<THREE>, or <FIVE>
                    5 if (out_digit | one).count_ones() == 5 => '3',
                    5 if (out_digit | four).count_ones() == 7 => '2',
                    5 => '5',
                    // count = 6 must be <ZERO>,<SIX>, or <NINE>
                    6 if (out_digit | one).count_ones() == 7 => '6',
                    6 if (out_digit | four).count_ones() == 6 => '9',
                    6 => '0',
                    _ => unreachable!(),
                }
            })
            .collect::<String>();
        sum += value.parse::<u32>().unwrap();
    }
    sum
}
