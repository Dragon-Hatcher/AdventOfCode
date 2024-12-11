use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 11)
}

fn dig_count(n: i64) -> u32 {
    n.ilog10() + 1
}

fn split_num(n: i64) -> (i64, i64) {
    let mask = 10i64.pow(dig_count(n) / 2);
    (n / mask, n % mask)
}

fn solve(input: &str, iters: i64) -> i64 {
    let mut stones: HashMap<i64, i64> = input.nums().map(|n| (n, 1)).collect();

    for _ in 0..iters {
        let mut new_stones = HashMap::default();

        for (stone, count) in stones {
            let mut add_stone = |n| *new_stones.entry(n).or_default() += count;

            if stone == 0 {
                add_stone(1);
            } else if dig_count(stone) % 2 == 0 {
                let (l, r) = split_num(stone);
                add_stone(l);
                add_stone(r);
            } else {
                add_stone(stone * 2024);
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
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
