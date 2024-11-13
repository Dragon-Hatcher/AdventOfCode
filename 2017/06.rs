use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 06)
}

fn solve(mut blocks: Vec<i64>) -> (i64, i64) {
    let mut seen = HashMap::new();

    for step in 0i64.. {
        if let Some(last_seen) = seen.get(&blocks) {
            return (step, step - last_seen);
        }
        seen.insert(blocks.clone(), step);

        let cnt = *blocks.iter().max().unwrap();
        let mut idx = blocks.iter().position(|b| *b == cnt).unwrap();
        blocks[idx] = 0;
        for _ in 0..cnt {
            idx += 1;
            idx %= blocks.len();
            blocks[idx] += 1;
        }
    }

    unreachable!()
}

fn part1(input: &str) -> i64 {
    solve(input.nums().collect()).0
}

fn part2(input: &str) -> i64 {
    solve(input.nums().collect()).1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "0 2 7 0";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 12841);
    assert_eq!(part2(input), 8038);
}
