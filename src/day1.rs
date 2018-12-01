extern crate utils;

mod data;
use data::DAY1_INPUT1;

use std::collections::BTreeSet;

use utils::wsv;

fn main() {
    println!("Output 1: {}", part1());
    println!("Output 2: {}", part2());
}

fn part1() -> isize {
    let input = wsv::<isize>(DAY1_INPUT1); // Load input as a whitespace seperated list.
    input.iter().sum()
}

fn part2() -> isize {
    let input = wsv::<isize>(DAY1_INPUT1); // Load input as a whitespace seperated list.
    let mut set = BTreeSet::new(); // BTreeSet seems to be faster than a HashSet.
    let mut acc = 0; // Setes the start value of the accumulator
    set.insert(0); // Insert the start value into the set.
    let cycle = input.iter().cycle(); // Use cycle to make a infinite iterator.
    for e in cycle {
        acc += e;
        if !set.insert(acc) {
            // acc did already exist in the set.
            return acc;
        }
    }
    unreachable!();
}
