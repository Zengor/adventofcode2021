pub fn part1(input: &str) -> i32 {
    let mut crabs: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.iter().map(|x| (x - median).abs()).sum()
}

pub fn part2(input: &str) -> impl ToString {
    let mut crabs: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    crabs.sort();
    let calc_fuel = |a: i32, b: i32| (a - b).abs() * ((a - b).abs() + 1) / 2;
    let mut lowest = std::i32::MAX;
    let (&start, &end) = (crabs.first().unwrap(), crabs.last().unwrap());
    'pivot: for pivot in start..=end {
        let mut curr_sum = 0;
        for crab in crabs.iter().filter(|x| **x != pivot) {
            curr_sum += calc_fuel(*crab, pivot);
            if curr_sum > lowest {
                continue 'pivot;
            }
        }
        if curr_sum < lowest {
            lowest = curr_sum;
        }
    }
    lowest
}
