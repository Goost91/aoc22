use std::collections::HashSet;

pub fn run_day3() {
    let input = include_str!("../inputs/3a.txt").lines().collect::<Vec<_>>();
    day3a(input.as_slice());
    day3b(input.as_slice());
}

fn parse_value(c: u8) -> u8 {
    if c > b'a' {
        return c - b'a' + 1;
    }
    return (c - b'A') + 27;
}

pub fn find_unique(line: &&str) -> char {
    let half = line.len() / 2;
    let left = &line[0..half].chars().collect::<HashSet<_>>();
    let right = &line[half..].chars().collect::<HashSet<_>>();
    let common = left.intersection(right).collect::<Vec<_>>();
    **common.first().unwrap()
}

pub fn day3a(input: &[&str]) {
    let total = input.iter()
        .fold(0, |total, line| {
            let val = parse_value(find_unique(line) as u8) as i32;
            total + val
        });

    println!("total 3a = {total}");
}

pub fn day3b(input: &[&str]) {
    let total: i32 = input
        .chunks(3)
        .map(|group| {
            let common = group[0].chars()
                .filter(|&c| group[1].contains(c) && group[2].contains(c))
                .next();
            parse_value(common.unwrap() as u8) as i32
        })
        .sum();

    println!("total 3b = {total}")
}