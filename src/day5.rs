const ALPHABEAT: &str = "abcdefghijklmnopqrstuvwxyz";

#[aoc(day5, part1, Simple)]
fn part1(input: &str) -> usize {
    let mut mut_str = String::from(input);
    let mut lowest_length = 1;
    let mut current_length = 0;
    while lowest_length != current_length {
        current_length = mut_str.len();
        for c in ALPHABEAT.chars() {
            mut_str = mut_str.replace(&format!("{}{}", c.to_ascii_uppercase(), c), "")
                             .replace(&format!("{}{}", c, c.to_ascii_uppercase()), "");
        }
        lowest_length = mut_str.len();
    }

    mut_str.trim().len()
}

#[aoc(day5, part2, Simple)]
fn part2(input: &str) -> usize {
    let mut smallest = 50000;
    for c in ALPHABEAT.chars() {
        let new_str = input.replace(c, "")
                           .replace(c.to_ascii_uppercase(), "");
        let res = part1(&new_str);
        if res < smallest {
            smallest = res;
        }
    }
    smallest
}
