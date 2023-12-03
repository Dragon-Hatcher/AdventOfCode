use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 22)
}

fn part1(input: &str) -> i64 {
    /*
player:
    hp: 50
    mana: 500
    shield: 6
    poson: 6
    recharge: 5
boss:
    hp: 51



    
     */

    todo!("Part 1")
}

fn part2(input: &str) -> i64 {
    todo!("Part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
