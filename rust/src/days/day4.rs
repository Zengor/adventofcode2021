use crate::util::Matrix;
use itertools::iproduct;
use std::collections::HashMap;

struct Grid {
    inner: Matrix<u32, 5, 5>,
    marked: HashMap<u32, bool>,
}

impl Grid {
    fn parse(raw: &str) -> Grid {
        let mut nums = raw.split_whitespace();
        let mut matrix = Matrix::empty();
        let mut marked = HashMap::new();
        for (y, x) in iproduct!(0..5, 0..5) {
            let value = nums
                .next()
                .expect("Not enough numbers to fill grid")
                .parse()
                .expect("Not a number");
            matrix[(x, y)] = value;
            marked.insert(value, false);
        }
        Grid {
            inner: matrix,
            marked,
        }
    }

    fn mark(&mut self, num: u32) {
        match self.marked.get_mut(&num) {
            Some(mark) => *mark = true,
            None => (),
        }
    }

    fn check_win(&self) -> bool {
        for row in 0..5 {
            let all = self.inner.row(row).all(|n| self.marked[n]);
            if all {
                return true;
            }
        }
        for col in 0..5 {
            let all = self.inner.col(col).all(|n| self.marked[n]);
            if all {
                return true;
            }
        }
        return false;
    }

    fn calculate_score(&self, last: u32) -> u32 {
        last * self
            .inner
            .all()
            .filter(|n| !self.marked.get(n).unwrap())
            .sum::<u32>()
    }
}

fn parse_input(input: &str) -> (Vec<Grid>, impl Iterator<Item = u32> + '_) {
    let mut sections = input.split("\n\n");
    let order = sections
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap());
    let grids: Vec<_> = sections.map(Grid::parse).collect();
    (grids, order)
}

pub fn part1(input: &str) -> impl ToString {
    let (mut grids, order) = parse_input(input);
    for number in order {
        for grid in grids.iter_mut() {
            grid.mark(number);
            if grid.check_win() {
                return grid.calculate_score(number);
            }
        }
    }
    unreachable!("Puzzle is assumed to have a solution");
}

pub fn part2(input: &str) -> impl ToString {
    let (mut grids, order) = parse_input(input);
    for number in order {
        for grid in grids.iter_mut() {
            grid.mark(number);
        }
        if grids.len() > 1 {
            grids.retain(|grid| !grid.check_win());
        } else {
            if grids[0].check_win() {
                return grids[0].calculate_score(number);
            }
        }
    }
    unreachable!("Puzzle is assumed to have a solution");
}
