mod data;
use data::DAY2_INPUT;

use std::fmt::Write;
use std::collections::HashMap;

fn main() {
    println!("Output 1: {}", part1());
    println!("Output 2: {}", part2());
}

fn part1() -> usize {
    let input = DAY2_INPUT.split_whitespace();
    let mut con2 = 0;
    let mut con3 = 0;
    for e in input {
        let mut c2 = false;
        let mut c3 = false;
        for i in e.chars() {
            let count = e.matches(i).count();
            match count {
                2 => c2 = true,
                3 => c3 = true,
                _ => (),
            }
        }
        if c2 { con2 += 1; }
        if c3 { con3 += 1; }
    }
    (con2 * con3)
}

fn part2() -> String {
    let mut firstmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut secondmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = DAY2_INPUT.split_whitespace();
    let fst = input.next().unwrap();
    let splt = fst.len()/2;
    {
        let (a, b) = fst.split_at(splt);
        firstmap.insert(a, vec![fst]);
        secondmap.insert(b, vec![fst]);
    }
    for e in input {
        let (a, b) = e.split_at(splt);
        firstmap.entry(a).and_modify(|v| v.push(e)).or_insert_with(|| vec![e]);
        secondmap.entry(b).and_modify(|v| v.push(e)).or_insert_with(|| vec![e]);
    }
    // https://users.rust-lang.org/t/composing-a-list-of-all-pairs/15475/3
    for e in firstmap.values() {
        if e.len() > 1 {
            //println!("{:?}", e);
            let pairs: Vec<(&str, &str)> = e.iter()
                .enumerate()
                .flat_map(move |t| std::iter::repeat(t.1).zip(e.iter().skip(t.0 + 1)))
                .map(|(a,b)| (*a, *b))
                .collect();
            for p in pairs {
                if is_one_off(p) {
                    return print_diff(p);
                }
            }
        }
    }
    for e in secondmap.values() {
        if e.len() > 1 {
            //println!("{:?}", e);
            let pairs: Vec<(&str, &str)> = e.iter()
                .enumerate()
                .flat_map(move |t| std::iter::repeat(t.1).zip(e.iter().skip(t.0 + 1)))
                .map(|(a,b)| (*a, *b))
                .collect();
            for p in pairs {
                if is_one_off(p) {
                    return print_diff(p);
                }
            }
        }
    }
    panic!("NONE FOUND!");
}

fn is_one_off(input: (&str, &str)) -> bool {
    let iter = input.0.chars().zip(input.1.chars());
    let mut one_diff = false;
    for e in iter {
        if e.0 != e.1 {
            if one_diff {
                return false;
            } else {
                one_diff = true;
            }
        }
    }
    true
}

fn print_diff(input: (&str, &str)) -> String {
    let mut ret_str = String::new();
    let iter = input.0.chars().zip(input.1.chars());
    for e in iter {
        if e.0 == e.1 {
            write!(ret_str, "{}",  e.0);
        }
    }
    ret_str
}
