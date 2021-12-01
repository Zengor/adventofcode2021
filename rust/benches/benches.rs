use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($group:ident, $name:expr, $func:expr, $input:expr) => {
        $group.bench_with_input($name, $input, |b, input| b.iter(|| $func(input)));
    };
}

macro_rules! bench_day {
    ($day:ident with input $input:literal) => {
        bench_day!($day => part1 part2 | with input $input)
    };
    ($day:ident => $($func:ident),+; with input $input:literal) => {
        fn $day(c: &mut Criterion) {
            let input = include_str!(concat!("../../input/", $input));
            let mut group = c.benchmark_group(stringify!($day));
            $(
                bench!(group, stringify!($func), aoc2021::$day::$func, input);
            )+
        }
    };
}

bench_day!(day1 => part1, part2, part2_noitertools;
           with input "01-1.txt");

criterion_group!(benches, day1);
criterion_main!(benches);
