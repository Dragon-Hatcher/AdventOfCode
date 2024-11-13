use advent::prelude::*;

#[derive(Debug, Clone)]
struct Card {
    guesses: FxHashSet<i64>,
    winners: FxHashSet<i64>,
}

impl Card {
    fn winning_numbers(&self) -> usize {
        self.guesses.intersection(&self.winners).count()
    }

    fn points(&self) -> i64 {
        let wins = self.winning_numbers() as u32;
        if wins == 0 { 0} else { 2i64.pow(wins - 1) }
    }
}

fn parse_card(card: &str) -> Card {
    let (_, numbers) = card.split_once(": ").unwrap();
    let (guesses, winners) = numbers.split_once(" | ").unwrap();
    let guesses = guesses.nums().collect();
    let winners = winners.nums().collect();
    Card { guesses, winners }
}

fn default_input() -> &'static str {
    include_input!(2023 / 04)
}

fn part1(input: &str) -> i64 {
    input.lines().map(parse_card).map(|c| c.points()).sum()
}

fn part2(input: &str) -> i64 {
    let win_counts = input
        .lines()
        .map(parse_card)
        .map(|c| c.winning_numbers())
        .collect_vec();

    let mut counts = vec![1; win_counts.len()];

    for (i, &wins) in win_counts.iter().enumerate() {
        let card_count = counts[i];
        for count in counts.iter_mut().skip(i + 1).take(wins) {
            *count += card_count;
        }
    }

    counts.iter().sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 30);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 21568);
    assert_eq!(part2(input), 11827296);
}
