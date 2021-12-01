use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    let mut iter = input
        .lines()
        .map(|l| l.trim().parse::<usize>().unwrap())
        .peekable();
    let mut count: usize = 0;
    while let (Some(curr), Some(next)) = (iter.next(), iter.peek()) {
        if next > &curr {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> impl ToString {
    let (_, total) = input
        .lines()
        .map(|l| l.trim().parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .fold((None, 0), |(prev_sum, total), curr_sum| {
            let total = if matches!(prev_sum, Some(prev) if curr_sum > prev) {
                total + 1
            } else {
                total
            };
            (Some(curr_sum), total)
        });
    total
}

pub fn part2_noitertools(input: &str) -> impl ToString {
    let numbers: Vec<usize> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    let mut prev_sum: usize = numbers[0..3].iter().sum();
    let mut total = 0;
    let length = numbers.len();
    for i in 3..=length {
        let curr = numbers[i - 3..i].iter().sum();
        if curr > prev_sum {
            total += 1;
        }
        prev_sum = curr;
    }
    total
}
