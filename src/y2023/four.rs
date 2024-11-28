use std::collections::HashMap;
use regex::Regex;

struct Card {
    winning_nums: Vec<i32>,
    card_nums: Vec<i32>,
    points: i32,
    matches: u16,
}

pub fn run() {
    let inp: String = std::fs::read_to_string("./input/2023/4t").unwrap();
    let re = Regex::new(r" +").unwrap();
    let inp = re.replace_all(inp.as_str(), " ").to_string();
    let mut total_points = 0;
    let mut cards: HashMap<u16, Card> = HashMap::new();

    for line in inp.split("\n") {
        let line = line.trim();
        let (card_num, nums) = line.split_once(":").unwrap();
        let card_num = card_num.split_once(" ").unwrap().1.parse::<u16>().unwrap();
        let (card_nums, winning_nums) = nums.split_once("|").unwrap();
        let card_nums = parse_nums(card_nums);
        let winning_nums = parse_nums(winning_nums);
        let mut matches = 0;
        for num in card_nums.iter() {
            if winning_nums.contains(num) {
                matches+=1;
            }
        }
        let mut points = if matches > 0 {1} else {0};
        for _ in 1..matches {
            points *= 2;
        }
        cards.insert(card_num, Card {
            card_nums,
            winning_nums,
            points,
            matches
        });
        total_points+=points;
    }
    let mut total = 0;
    for (i, card) in cards.iter() {
        total += count_scratchcards(card, &cards, *i);
    }
    println!("{total}");
}

fn parse_nums(s: &str) -> Vec<i32> {
    s.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn count_scratchcards(c: &Card, cards: &HashMap<u16, Card>, i: u16) -> u32 {
    let mut count = 1;
    for j in 1..=c.matches {
        count += count_scratchcards(cards.get(&(i+j)).unwrap(), &cards, i+j);
    }
    count
}
