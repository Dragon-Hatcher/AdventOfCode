use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 02)
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn max_allowed(self) -> i64 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

fn parse_item(item: &str) -> (i64, Color) {
    let count = item.nums().nu();
    let color = if item.ends_with("red") {
        Color::Red
    } else if item.ends_with("green") {
        Color::Green
    } else {
        Color::Blue
    };

    (count, color)
}

fn parse_line(line: &str) -> (i64, impl Iterator<Item = (i64, Color)> + '_) {
    let id = line.nums().nu();
    let (_, items) = line.split_once(':').unwrap();
    (id, items.split([';', ',']).map(parse_item))
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (id, mut items) = parse_line(l);
            let impossible = items.any(|(count, color)| count > color.max_allowed());

            if impossible {
                0
            } else {
                id
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (_, items) = parse_line(l);

            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for (count, color) in items {
                match color {
                    Color::Red => red = red.max(count),
                    Color::Green => blue = blue.max(count),
                    Color::Blue => green = green.max(count),
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
fn example1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(input), 8);
    assert_eq!(part2(input), 2286);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2486);
    assert_eq!(part2(input), 87984);
}
