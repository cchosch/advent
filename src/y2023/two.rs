use std::collections::HashMap;

pub fn run() {
    let inp: String = std::fs::read_to_string("./input/2023/2").unwrap();
    let maximum = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut game_min = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);
    let mut game_ids = 0;
    for line in inp.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line = line.trim();
        let (g, l) = line.split_once(":").unwrap();
        game_min.insert("red", 0);
        game_min.insert("green", 0);
        game_min.insert("blue", 0);

        for session in l.split(";") {
            for item in session.split(",") {
                let (count, color) = item.trim().split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                if game_min.get(color).unwrap() < &count {
                    game_min.insert(color, count);
                }
            }
        }
        let game_count = game_min.iter().map(|(_, x)| x).product::<i32>();
        game_ids += game_count;
    }

    println!("{}", game_ids);
}
