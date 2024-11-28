use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::Chars;

#[derive(Debug)]
enum ArrState {
    None,
    Symbol,
    Number(usize)
}

pub fn run() {
    let inp: String = std::fs::read_to_string("./input/2023/3").unwrap();
    let mut part_indexes: HashSet<usize> = HashSet::new();
    let mut numbers: Vec<u32> = vec![];
    let mut symbols: Vec<((usize, usize), char)> = vec![];
    let mut positions: Vec<Vec<ArrState>> = vec![];

    let mut j: usize = 0;
    for line in inp.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line = line.trim();
        let mut c_line = vec![];
        let chars = line.chars().collect::<Vec<char>>();
        for (i, c) in chars.iter().enumerate() {
            let to_push = match c {
                '.' => ArrState::None,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if i > 0 && to_number(&chars[i-1]).is_some() {
                        c_line.push(ArrState::Number(numbers.len()-1));
                        continue;
                    }
                    let mut number = vec![];
                    for k in i..line.len() {
                        match to_number(&chars[k]) {
                            None => {
                                break
                            },
                            Some(num) => {
                                number.insert(0, num);
                            }
                        }
                    }
                    numbers.push(
                        number.into_iter().enumerate().map(|(i, n)| n * 10u32.pow(i as u32)).sum::<u32>(),
                    );

                    ArrState::Number(numbers.len()-1)
                },
                c => {
                    symbols.push(((j, i), c.clone()));
                    ArrState::Symbol
                }
            };
            c_line.push(to_push);
        }
        positions.push(c_line);
        j += 1;
    }

    let mut total = 0;
    for ((y, x), c) in symbols {
        if c != '*' {
            continue;
        }
        let mut touching = vec![];
        for dx in -1..=1 {
            if (x as i32 + dx) as usize >= positions[0].len() {
                continue
            }
            for dy in -1..=1 {
                if (y as i32 + dy) as usize >= positions.len() {
                    continue
                }
                if dx == 0 && dy == 0 {
                    continue;
                }
                // println!("{}, {}", x as i32 + dx, y as i32 + dy);
                match positions[(y as i32 + dy) as usize][(x as i32 + dx) as usize] {
                    ArrState::None => {},
                    ArrState::Symbol => {},
                    ArrState::Number(x) => {
                        if !touching.contains(&x) {
                            touching.push(x);
                        }
                    }
                }
            }
        }
        if touching.len() == 2 {
            total += numbers[touching[0]]*numbers[touching[1]];
        }
    }
    let mut sum = 0;
    for part_i in part_indexes.iter() {
        sum += numbers[*part_i];
    }
    println!("{total}");
}

fn to_number(c: &char) -> Option<u32> {
    return match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    };
}
