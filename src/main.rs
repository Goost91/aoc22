use crate::day1::{day1a, day1b, run_day1};
use brunch::{Bench, benches};

mod day1;

fn main() {
    //println!("Hello, world!");
    run_day1();

    let day1 = include_str!("../inputs/1a.txt").lines();

    benches!(
        inline:

        Bench::new("day1a").run_seeded_with(|| day1.clone(), |input| { day1a(input) }),
        Bench::new("day1b").run_seeded_with(|| day1.clone(), |input| { day1b(input)}),
    )
}
