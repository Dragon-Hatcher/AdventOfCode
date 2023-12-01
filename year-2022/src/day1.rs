use std::collections::HashMap;

fn find(line: &str, char_map: &HashMap<String, i64>) -> i64 {
    *char_map
        .iter()
        .map(|(k, v)| (line.find(k), v))
        .filter(|(f, _)| f.is_some())
        .min_by_key(|(f, _)| *f)
        .unwrap()
        .1
}

fn solve(line: &str, char_map: &HashMap<String, i64>) -> i64 {
    let first = find(line, char_map);

    let mut rev_map = HashMap::new();
    for (k, v) in char_map { rev_map.insert(k.chars().rev().collect::<String>(), *v); }

    let second = find(&line.chars().rev().collect::<String>(), &rev_map);

    first * 10 + second
}

pub fn part1(input: &str) -> i64 {
    let mut map: HashMap<String, i64> = HashMap::new();
    map.insert("0".to_owned(), 0);
    map.insert("1".to_owned(), 1);
    map.insert("2".to_owned(), 2);
    map.insert("3".to_owned(), 3);
    map.insert("4".to_owned(), 4);
    map.insert("5".to_owned(), 5);
    map.insert("6".to_owned(), 6);
    map.insert("7".to_owned(), 7);
    map.insert("8".to_owned(), 8);
    map.insert("9".to_owned(), 9);
    
    input
        .lines()
        .map(|l| solve(l, &map))
        .sum()
}


pub fn part2(input: &str) -> i64 {
    let mut map: HashMap<String, i64> = HashMap::new();
    map.insert("0".to_owned(), 0);
    map.insert("1".to_owned(), 1);
    map.insert("2".to_owned(), 2);
    map.insert("3".to_owned(), 3);
    map.insert("4".to_owned(), 4);
    map.insert("5".to_owned(), 5);
    map.insert("6".to_owned(), 6);
    map.insert("7".to_owned(), 7);
    map.insert("8".to_owned(), 8);
    map.insert("9".to_owned(), 9);
    map.insert("zero".to_owned(), 0);
    map.insert("one".to_owned(), 1);
    map.insert("two".to_owned(), 2);
    map.insert("three".to_owned(), 3);
    map.insert("four".to_owned(), 4);
    map.insert("five".to_owned(), 5);
    map.insert("six".to_owned(), 6);
    map.insert("seven".to_owned(), 7);
    map.insert("eight".to_owned(), 8);
    map.insert("nine".to_owned(), 9);
    
    input
        .lines()
        .map(|l| solve(l, &map))
        .sum()
}

const PART1_EX_ANSWER: &str = "142";
const PART1_ANSWER: &str = "54632";
const PART2_EX_ANSWER: &str = "142";
const PART2_ANSWER: &str = "54019";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
