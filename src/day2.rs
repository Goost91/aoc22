use std::collections::HashMap;
use std::str::Lines;
use lazy_static::lazy_static;

lazy_static! {
    static ref HANDS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("A X", 4); m.insert("B X", 1);  m.insert("C X", 7);
        m.insert("A Y", 8); m.insert("B Y", 5); m.insert("C Y", 2);
        m.insert("A Z", 3); m.insert("B Z", 9); m.insert("C Z", 6);
        m
    };

    static ref HANDS2: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("A X", 3); m.insert("B X", 1);  m.insert("C X", 2);
        m.insert("A Y", 4); m.insert("B Y", 5); m.insert("C Y", 6);
        m.insert("A Z", 8); m.insert("B Z", 9); m.insert("C Z", 7);
        m
    };
}
pub fn run_day2() {
    let input = include_str!("../inputs/2a.txt").lines().collect::<Vec<_>>();
    day2a(input.as_slice());
    day2b(input.as_slice());
}

pub fn day2a(lines: &[&str]) {
    let result = lines.iter()
        .fold(0, |total, line| {
            total + HANDS.get(line).unwrap()
        });

    println!("{result}");
}

pub fn day2b(lines: &[&str]) {
    let result = lines.iter()
        .fold(0, |total, line| {
            total + HANDS2.get(line).unwrap()
        });

    println!("{result}");
}