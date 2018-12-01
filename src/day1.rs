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
    let input = wsv::<isize>(DAY1_INPUT1);
    let iter = input.iter();
    iter.fold(0, |acc, e| acc + e)
}

fn part2() -> isize {
    let input = wsv::<isize>(DAY1_INPUT1);
    let mut set = BTreeSet::new();
    let mut counter = 0;
    let mut acc = 0;
    loop {
        let iter = input.iter();
        for e in iter {
            acc += e;
            if !set.insert(acc) {
                return acc;
            }
        }
    }
}
