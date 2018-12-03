use regex::Regex;

#[aoc(day3, part1, Simple)]
fn part1(input: &str) -> usize {
    let mut fabric: Vec<Vec<usize>> = Vec::new();
    for _ in 0..1000 {
        fabric.push(vec![0; 1000]);
    }
    let lines = input.split('\n');
    for line in lines {
        let (_, (x, y), (w, h)) = fabric_parser(line);
        for x_row in x..(x+w) {
            for y_col in y..(y+h) {
                fabric[x_row][y_col] += 1;
            }
        }
    }
        
    let mut more_than_one = 0;
    for e in fabric.iter().flatten() {
        if e >= &2 {
            more_than_one += 1;
        }
    }
    more_than_one
}

#[aoc(day3, part2, Simple)]
fn part2(input: &str) -> usize {
    let mut fabric: Vec<Vec<usize>> = Vec::new();
    for _ in 0..1000 {
        fabric.push(vec![0; 1000]);
    }
    let mut claims: Vec<(bool)> = vec![false; 2000];
    let lines = input.split('\n');
    for line in lines {
        let (id, (x, y), (w, h)) = fabric_parser(line);
        claims[id] = true;
        for x_row in x..(x+w) {
            for y_col in y..(y+h) {
                fabric[x_row][y_col] += id;
            }
        }
    }
    let lines_2 = input.split('\n');
    for line in lines_2 {
        let (id, (x, y), (w, h)) = fabric_parser(line);
        for x_row in x..(x+w) {
            for y_col in y..(y+h) {
                if fabric[x_row][y_col] != id {
                    claims[id] = false;
                }
            }
        }
    }
    for (i, claim) in claims.iter().enumerate() {
        if *claim && i != 0 {
            return i
        }
    }
    0
}

lazy_static! {
    static ref FABRIC_RE: Regex = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
}

// #1 @ 1,3: 4x4
fn fabric_parser(input: &str) -> (usize, (usize, usize), (usize,usize)) {
    let caps = FABRIC_RE.captures(input);
    match caps {
        None => {
            let inner = (0,0);
            (0, inner, inner)
        },
        Some(cap) => {
            (cap[1].parse::<usize>().expect("parse error"),
             (cap[2].parse::<usize>().expect("parse error"), cap[3].parse::<usize>().expect("parse error")),
             (cap[4].parse::<usize>().expect("parse error"), cap[5].parse::<usize>().expect("parse error")))
        },
    }
}
