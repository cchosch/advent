use std::collections::{HashMap, HashSet};

pub fn run() {
    let fl = std::fs::read_to_string("./input/2024/1").unwrap();
    let mut left = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();

    for ln in fl.split("\n") {
        if ln.len() == 0 {
            continue
        }
        let (l, r) = ln.trim().split_once(" ").unwrap();
        left.push(l.trim().parse::<u32>().unwrap());
        let r = r.trim().parse::<u32>().unwrap();
        if right.contains_key(&r) {
            right.insert(r, right.get(&r).unwrap()+1);
        } else {
            right.insert(r, 1);
        }
    }
    left.sort();
    let mut diff = 0;
    for num in left {
        match right.get(&num) {
            None => continue,
            Some(c) => {
                diff += num * c
            }
        }
    }
    println!("{diff}")
}
