use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 02)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            // let id = l.nums().nu();

            let newl = l.chars().skip(8).collect::<String>();
            let iter = newl.split([';', ',']);
            for item in iter {
                // dbg!(1, item);

                if item.ends_with("blue") {
                    if item.nums().nu() > 14 {
                        return 0;
                    }
                }

                if item.ends_with("red") {
                    if item.nums().nu() > 12 {
                        return 0;
                    }
                }

                if item.ends_with("green") {
                    if item.nums().nu() > 13 {
                        return 0;
                    }
                }
            }

            l.nums().nu()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            // let id = l.nums().nu();

            let newl = l.chars().skip(8).collect::<String>();
            let iter = newl.split([';', ',']);

            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for item in iter {
                // dbg!(1, item);

                if item.ends_with("blue") {
                    blue = blue.max(item.nums().nu());
                }

                if item.ends_with("red") {
                    red = red.max(item.nums().nu());
                }

                if item.ends_with("green") {
                    green = green.max(item.nums().nu());
                }
            }

            red * green * blue
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    // assert_eq!(part1(input), 8);
    assert_eq!(part2(input), 0);
}

// #[test]
// fn default() {
//     let input = default_input();
//     assert_eq!(part1(input), 0);
//     assert_eq!(part2(input), 0);
// }
