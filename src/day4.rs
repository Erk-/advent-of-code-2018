use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    let mut string_vec: Vec<String> = input.lines().map(|e| e.to_owned()).collect();
    string_vec.sort_unstable();
    string_vec
}

#[aoc(day4, part1, Simple)]
fn part1(input: &[String]) -> usize {
    let iter = input.iter();
    let mut guards: HashMap<usize, Vec<Vec<bool>>> = HashMap::new();
    let mut current_guard: (usize, Vec<bool>) = (0, vec![false; 60]);
    let mut asleep_at = None;
    for line in iter {
        if is_guard(line) {
            if let Some(mm) = asleep_at {
                for i in current_guard.1[mm..].iter_mut() {
                    *i = true;
                }
            }
            guards.entry(current_guard.0).and_modify(|v| v.push(current_guard.1.clone())).or_insert_with(|| vec![current_guard.1]);
            //guards.push(current_guard);
            let id = get_guard(line);
            current_guard = (id, vec![false; 60]);
        } else if let Some(mm) = is_falls(line) {
            asleep_at = Some(mm);
        } else if let Some(mm) = is_wakes(line) {
            if let Some(aa) = asleep_at {
                for i in current_guard.1[aa..mm].iter_mut() {
                    *i = true;
                }
                asleep_at = None;
            } else {
                unreachable!();
            }
        }
    }
    //guards.sort_by_key(|k| k.0);
    let mut guard_sleep: HashMap<usize, usize> = HashMap::new();
    for (key, val) in guards.iter() {
        for v in val {
            guard_sleep.entry(*key)
                .and_modify(|e| *e += count_sleep_total(v))
                .or_insert_with(|| count_sleep_total(v));
        }
    }
    let sleepy_guard = guard_sleep.iter().max_by_key(|e| e.1).unwrap();
    let mut when = vec![0;60];
    for e in guards.get(sleepy_guard.0).iter() {
        for f in e.iter() {
            for (i, g) in f.iter().enumerate() {
                when[i] += if *g {1} else {0};
            }
        }
    }
    
    let mut max_index = 0;
    let mut max = 0;
    for (i, e) in when.iter().enumerate() {
        if *e > max {
            max_index = i;
            max = *e;
        }
    }

    (max_index * sleepy_guard.0)
}

#[aoc(day4, part2, Simple)]
fn part2(input: &[String]) -> usize {
    let iter = input.iter();
    let mut guards: HashMap<usize, Vec<Vec<bool>>> = HashMap::new();
    let mut current_guard: (usize, Vec<bool>) = (0, vec![false; 60]);
    let mut asleep_at = None;
    for line in iter {
        if is_guard(line) {
            if let Some(mm) = asleep_at {
                for i in current_guard.1[mm..].iter_mut() {
                    *i = true;
                }
            }
            guards.entry(current_guard.0).and_modify(|v| v.push(current_guard.1.clone())).or_insert_with(|| vec![current_guard.1]);
            let id = get_guard(line);
            current_guard = (id, vec![false; 60]);
        } else if let Some(mm) = is_falls(line) {
            asleep_at = Some(mm);
        } else if let Some(mm) = is_wakes(line) {
            if let Some(aa) = asleep_at {
                for i in current_guard.1[aa..mm].iter_mut() {
                    *i = true;
                }
                asleep_at = None;
            } else {
                unreachable!();
            }
        }
    }

    let mut toptop: HashMap<usize, (usize, usize)> = HashMap::new();
    for (key, val) in guards.iter() {
        toptop.insert(*key, max_index(sum_vec_vec(val)));
    }
    
    let sleepy_guard = toptop.iter().max_by_key(|e| (e.1).1).unwrap();
    
    ((sleepy_guard.1).0 * sleepy_guard.0)
}

fn max_index(lst: Vec<usize>) -> (usize, usize) {
    let mut max_index = 0;
    let mut max = 0;
    for (i, e) in lst.iter().enumerate() {
        if *e > max {
            max_index = i;
            max = *e;
        }
    }
    (max_index, max)
}

fn sum_vec_vec(lst: &[Vec<bool>]) -> Vec<usize> {
    let mut arr = vec![0;60];
    for e in lst.iter() {
        for (i, f) in e.iter().enumerate() {
            arr[i] += if *f {1} else {0};
        }
    }
    arr
}

#[inline]
fn count_sleep_total(arr: &[bool]) -> usize {
    arr.iter().fold(0, |acc, e| if *e {acc + 1} else {acc})
}

#[inline]
fn is_guard(input: &str) -> bool {
    input.contains("Guard")
}

#[inline]        
fn get_guard(input: &str) -> usize {
    input.split_ascii_whitespace()
        .nth(3).unwrap()
        .get(1..).unwrap()
        .parse::<usize>().unwrap()
}

#[inline]
fn is_falls(input: &str) -> Option<usize> {
    if input.contains("falls") { Some(input.get(15..17).unwrap().parse::<usize>().unwrap()) } else { None }
}

#[inline]
fn is_wakes(input: &str) -> Option<usize> {
    if input.contains("wakes") { Some(input.get(15..17).unwrap().parse::<usize>().unwrap()) } else { None }
}
