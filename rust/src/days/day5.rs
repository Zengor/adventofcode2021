use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    line.split("->")
        .map(|p| {
            p.trim()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

pub fn part1(input: &str) -> impl ToString {
    let mut points: HashMap<(u32, u32), u32> = HashMap::new();
    for line in input.lines() {
        let (from, to) = parse_line(line);
        if from.0 == to.0 {
            let start = min(from.1, to.1);
            let end = max(from.1, to.1);
            for n in start..=end {
                *points.entry((from.0, n)).or_default() += 1u32;
            }
        } else if from.1 == to.1 {
            let start = min(from.0, to.0);
            let end = max(from.0, to.0);
            for n in start..=end {
                *points.entry((n, from.1)).or_default() += 1u32;
            }
        }
    }
    points.values().filter(|count| **count > 1).count()
}

pub fn part2(input: &str) -> impl ToString {
    use std::cmp::Ordering;
    let mut points: HashMap<(u32, u32), u32> = HashMap::new();
    for line in input.lines() {
        let (from, to) = parse_line(line);
        let move_x = match from.0.cmp(&to.0) {
            Ordering::Equal => |x| x,
            Ordering::Greater => |x| x - 1,
            Ordering::Less => |x| x + 1,
        };
        let move_y = match from.1.cmp(&to.1) {
            Ordering::Equal => |y| y,
            Ordering::Greater => |y| y - 1,
            Ordering::Less => |y| y + 1,
        };
        let (mut curr_x, mut curr_y) = (from.0, from.1);
        while (curr_x, curr_y) != (to.0, to.1) {
            *points.entry((curr_x, curr_y)).or_default() += 1;
            curr_x = move_x(curr_x);
            curr_y = move_y(curr_y);
        }
        *points.entry((to.0, to.1)).or_default() += 1;
    }
    points.values().filter(|count| **count > 1).count()
}
