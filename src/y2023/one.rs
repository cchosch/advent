pub fn run() {
    let fl = std::fs::read_to_string("./input").unwrap();
    let mut results = vec![];
    let find_replace: Vec<(&[u8], i32)> = vec![
        (b"1", 1),
        (b"2", 2),
        (b"3", 3),
        (b"4", 4),
        (b"5", 5),
        (b"6", 6),
        (b"7", 7),
        (b"8", 8),
        (b"9", 9),
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];
    for line in fl.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut numbers = map_index(line.as_bytes(), &find_replace);
        if numbers.len() == 1 {
            numbers.push(numbers[0]);
        }
        results.push((numbers[0] * 10) + numbers.last().unwrap());
    }
    println!("{}", results.iter().sum::<i32>());
}

fn map_index(arr: &[u8], find_replace: &Vec<(&[u8], i32)>) -> Vec<i32> {
    let mut values = vec![];
    for i in 0..arr.len() {
        for (y, map_to) in find_replace.iter() {
            if i+y.len()-1 < arr.len() {
                let mut worked = true;
                for j in 0..y.len() {
                    if arr[i+j] != y[j] {
                        worked = false;
                        break
                    }
                }
                if worked {
                    values.push(*map_to);
                }
            }
        }
    }
    return values;
}
