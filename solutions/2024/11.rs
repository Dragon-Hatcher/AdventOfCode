use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 11)
}

fn solve(input: &str, iters: i64) -> i64 {
    let stones_list = input.nums().collect_vec();
    let mut table = HashMap::default();

    for stone in stones_list {
        *table.entry(stone).or_default() += 1;
    }

    for _ in 0..iters {
        let mut new = HashMap::default();

        for (stone, count) in table {
            if stone == 0 {
                *new.entry(1).or_default() += count;
            } else if stone.to_string().len() % 2 == 0 {
                let len = stone.to_string().len() / 2;
                *new.entry(stone.to_string().chars().take(len).collect::<String>().nums().nu()).or_default() += count;
                *new.entry(stone.to_string().chars().skip(len).collect::<String>().nums().nu()).or_default() += count;
            } else {
                *new.entry(stone * 2024).or_default() += count;
            }
        }

        table = new;   
    }

    table.values().sum::<i64>()
}

fn part1(input: &str) -> i64 {
    solve(input, 25)    
}

fn part2(input: &str) -> i64 {
    solve(input, 75)
}

fn main() {
    advent::new(2024, 11, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "125 17";
    assert_eq!(solve(input, 25), 55312);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 200446);
    assert_eq!(part2(input), 238317474993392);
}
