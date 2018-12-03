use crate::utils::wsv;

use std::collections::BTreeSet;

#[aoc(day1, part1, Simple)]
fn part1(input: &str) -> isize {
    let in_arr = wsv::<isize>(input); // Load input as a whitespace seperated list.
    in_arr.iter().sum()
}

#[aoc(day1, part2, Simple)]
fn part2(input: &str) -> isize {
    let in_arr = wsv::<isize>(input); // Load input as a whitespace seperated list.
    let mut set = BTreeSet::new(); // BTreeSet seems to be faster than a HashSet.
    let mut acc = 0; // Setes the start value of the accumulator
    set.insert(0); // Insert the start value into the set.
    let cycle = in_arr.iter().cycle(); // Use cycle to make a infinite iterator.
    for e in cycle {
        acc += e;
        if !set.insert(acc) {
            // acc did already exist in the set.
            return acc;
        }
    }
    unreachable!();
}
