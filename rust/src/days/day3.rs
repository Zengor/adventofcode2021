pub fn ones_count_per_column<const N: usize>(input: &str) -> [usize; N] {
    let mut counts = [0usize; N];
    for line in input.lines() {
        for (i, bit) in line.trim().chars().enumerate() {
            match bit {
                '1' => counts[i] += 1,
                '0' => (), // just need to count either of them to know the other
                _ => panic!("invalid input"),
            }
        }
    }
    counts
}
// the problem input only has 12-bit numbers.
// However this isn't actually specified in the problem statement
// and I wanted to test with the examples as well so it felt like a cool place to take advantage of const generics,
// even if absolutely unnecessary
pub fn part1<const N: usize>(input: &str) -> u32 {
    let num_lines = input.lines().count();
    let ones_greater = ones_count_per_column::<N>(input)
        .into_iter()
        .map(|c| if c > num_lines / 2 { true } else { false })
        .rev()
        .enumerate();
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (i, ones_most_common) in ones_greater {
        if ones_most_common {
            gamma_rate = gamma_rate | (1 << i);
        } else {
            epsilon_rate = epsilon_rate | (1 << i);
        }
    }
    gamma_rate * epsilon_rate
}

fn most_common(lines: &[&str], column: usize) -> char {
    let mut ones = 0;
    let mut zeroes = 0;
    for line in lines.iter() {
        match line.chars().nth(column).unwrap() {
            '1' => ones += 1,
            '0' => zeroes += 1,
            _ => panic!(),
        }
    }
    if ones >= zeroes {
        '1'
    } else {
        '0'
    }
}

pub fn part2<const N: usize>(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut generator_candidates = lines.clone();
    let mut scrubber_candidates = lines;
    for column in 0..N {
        dbg!(&generator_candidates);
        dbg!(&scrubber_candidates);
        let generator_common = dbg!(most_common(&generator_candidates, column));
        let scrubber_common = dbg!(most_common(&scrubber_candidates, column));
        if generator_candidates.len() > 1 {
            generator_candidates.retain(|l| l.chars().nth(column).unwrap() == generator_common);
        }
        if scrubber_candidates.len() > 1 {
            scrubber_candidates.retain(|l| l.chars().nth(column).unwrap() != scrubber_common);
        }
    }
    let generator_rating: u32 = generator_candidates
        .first()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .expect("No solution found");
    let scrubber_rating: u32 = scrubber_candidates
        .first()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .expect("No solution found");
    dbg!(generator_rating) * dbg!(scrubber_rating)
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
00010
11001
01010";
    assert_eq!(part1::<5>(input), 198u32);
}
