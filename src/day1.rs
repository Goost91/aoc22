use std::num::ParseIntError;
use std::str::Lines;

pub fn run_day1() {

    let input = include_str!("../inputs/1a.txt").lines();
    let a = day1a(input.clone());
    let b = day1b(input.clone());

    println!("part 1 {} part 2 {}", a.0, a.1);
    println!("part 1 {} part 2 {}", b.0, b.1);
}

pub fn day1a(lines: Lines) -> (i32, i32) {
    //let input = include_str!("../inputs/1a.txt").lines();
    let mut values = Vec::new();

    let last = lines.fold(0, |acc, x| {
        match x.parse::<i32>() {
            Ok(value) => { acc + value }
            Err(_) => {
                values.push(acc);
                0
            }
        }
    });

    values.push(last);
    values.sort_by(|a, b| b.cmp(a));

//    println!("{}", values[0]);
  //  println!("{}", values[0..=2].iter().sum::<i32>());
    return (values[0], values[0..=2].iter().sum::<i32>());
}

pub fn day1b(lines: Lines) -> (i32, i32) {
    let input = lines.map(|x| x.parse::<i32>()).collect::<Vec<_>>();
    let result = solve(0, 0, 0, 0, input.as_slice());

   // println!("{}", result.0);
   // println!("{}", result.1);
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
