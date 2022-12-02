use std::num::ParseIntError;
use std::slice::Iter;
use std::str::Lines;
use brunch::{Bench, benches};

pub fn run_day1() {
    let input = include_str!("../inputs/1a.txt").lines();
    let a = day1a(input.clone().map(|x| x.parse::<i32>()).collect::<Vec<_>>().iter());
    let b = day1b(input.clone().map(|x| x.parse::<i32>()).collect::<Vec<_>>().as_slice());

    println!("part 1 {} part 2 {}", a.0, a.1);
    println!("part 1 {} part 2 {}", b.0, b.1);
}

pub fn bench_day1() {
    let day1 = include_str!("../inputs/1a.txt").lines().clone().map(|x| x.parse::<i32>()).collect::<Vec<_>>();

    benches!(
        inline:

        Bench::new("day1a").run_seeded_with(|| day1.clone(), |input| { day1a(input.iter()) }),
        Bench::new("day1b").run_seeded_with(|| day1.clone(), |input| { day1b(input.as_slice())}),
    )
}

pub fn day1a(lines: Iter<Result<i32, ParseIntError>>) -> (i32, i32) {
    //let input = include_str!("../inputs/1a.txt").lines();
    let mut values = Vec::new();

    let last = lines.fold(0, |acc, x| {
        match x {
            Ok(value) => { acc + value }
            Err(_) => {
                values.push(acc);
                0
            }
        }
    });

    values.push(last);
    values.sort_by(|a, b| b.cmp(a));

    return (values[0], values[0..=2].iter().sum::<i32>());
}

pub fn day1b(lines: &[Result<i32, ParseIntError>]) -> (i32, i32) {
    let input = lines;
    let result = solve(0, 0, 0, 0, input);

    return result;
}

fn solve(acc: i32, a: i32, b: i32, c: i32, values: &[Result<i32, ParseIntError>]) -> (i32, i32) {
    match values {
        [Err(_), tail @ ..] if acc > a => solve(0, acc, a, b, tail),
        [Err(_), tail @ ..] if acc > b => solve(0, a, acc, b, tail),
        [Err(_), tail @ ..] if acc > c => solve(0, a, b, acc, tail),
        [Err(_), tail @ ..] => solve(0, a, b, c, tail),
        [Ok(val), tail @ ..] => solve(acc + val, a, b, c, tail),
        [] => (a, a + b + c),
    }
}

