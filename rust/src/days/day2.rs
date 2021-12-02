use itertools::Itertools;

struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn with_move(mut self, (dir, val): (&str, u32)) -> Self {
        match dir {
            "forward" => self.x += val,
            "down" => self.y += val,
            "up" => self.y -= val,
            _ => panic!("Invalid input"),
        }
        self
    }
    fn with_aim_move(mut self, (dir, val): (&str, u32)) -> Self {
        match dir {
            "forward" => {
                self.x += val;
                self.y += self.aim * val
            }
            "down" => self.aim += val,
            "up" => self.aim -= val,
            _ => panic!("Invalid input"),
        }
        self
    }

    fn pos_value(&self) -> u32 {
        self.x * self.y
    }
}

fn instructions(input: &str) -> impl Iterator<Item = (&str, u32)> {
    input
        .lines()
        .map(|line| line.trim().split(' ').collect_tuple().unwrap())
        .map(|(dir, val)| (dir, val.parse::<u32>().unwrap()))
}

pub fn part1(input: &str) -> impl ToString {
    instructions(input)
        .fold(Position::new(), Position::with_move)
        .pos_value()
}

pub fn part2(input: &str) -> impl ToString {
    instructions(input)
        .fold(Position::new(), Position::with_aim_move)
        .pos_value()
}
