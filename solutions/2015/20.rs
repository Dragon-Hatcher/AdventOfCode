use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 20)
}

fn part1(input: &str) -> i64 {
    fn presents(house: i64) -> i64 {
        house.divisors().sum::<i64>() * 10
    }

    let goal = input.nums().nu();
    for house in 2.. {
        if presents(house) >= goal {
            return house;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i64 {
    fn presents(house: i64) -> i64 {
        house.divisors().filter(|d| house <= d * 50).sum::<i64>() * 11
    }

    let goal = input.nums().nu();
    for house in 2.. {
        if presents(house) >= goal {
            return house;
        }
    }

    unreachable!()
}

fn main() {
    advent::new(2015, 20, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    assert_eq!(part1("140"), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 665280);
    assert_eq!(part2(input), 705600);
}
