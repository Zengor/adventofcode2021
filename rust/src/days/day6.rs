use std::collections::VecDeque;

fn fish_simulation(input: &str, steps: u32) -> u64 {
    let mut age_groups = VecDeque::from([0u64; 9]);
    for i in input.split(",") {
        let i = i.trim().parse().unwrap();
        age_groups[i] += 1;
    }
    for _ in 0..steps {
        let births = age_groups.pop_front().unwrap();
        age_groups[6] += births;
        age_groups.push_back(births);
    }
    age_groups.iter().sum::<u64>()
}

pub fn part1(input: &str) -> impl ToString {
    fish_simulation(input, 80)
}

pub fn part2(input: &str) -> impl ToString {
    fish_simulation(input, 256)
}
