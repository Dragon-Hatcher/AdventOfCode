use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 07)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
    Num(i64),
}

fn parse_card(c: char, jokers: bool) -> Card {
    match c {
        c if c.is_ascii_digit() => Card::Num(c as i64 - '0' as i64),
        'A' => Card::Num(14),
        'K' => Card::Num(13),
        'Q' => Card::Num(12),
        'J' => {
            if jokers {
                Card::Joker
            } else {
                Card::Num(11)
            }
        }
        'T' => Card::Num(10),
        _ => panic!("Invalid card {c}."),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    fn hand_type(self) -> HandType {
        let mut counts: HashMap<Card, i64> = HashMap::new();
        for card in self.0 {
            *counts.entry(card).or_default() += 1;
        }

        let joker_count = counts.get(&Card::Joker).copied().unwrap_or_default();
        counts.remove(&Card::Joker);

        let mut counts = counts.values().copied().sorted_by_key(|c| Reverse(*c));
        let best = counts.next().unwrap_or_default() + joker_count;
        let second = counts.next().unwrap_or_default();

        match (best, second) {
            (5, _) => HandType::Five,
            (4, _) => HandType::Four,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::Three,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::Pair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hand_type(), self.0).cmp(&(other.hand_type(), other.0))
    }
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    bet: i64,
}

fn parse_round(str: &str, jokers: bool) -> Round {
    let (hand, bet) = str.split_once(' ').unwrap();
    let hand = hand.chars().map(|c| parse_card(c, jokers)).collect_vec();
    let hand = Hand(hand.try_into().unwrap());
    let bet = bet.parse().unwrap();
    Round { hand, bet }
}

fn solve(input: &str, jokers: bool) -> i64 {
    input
        .lines()
        .map(|l| parse_round(l, jokers))
        .sorted_by_key(|r| r.hand)
        .enumerate()
        .map(|(i, r)| (i as i64 + 1) * r.bet)
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, false)
}

fn part2(input: &str) -> i64 {
    solve(input, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(input), 6440);
    assert_eq!(part2(input), 5905);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 251058093);
    assert_eq!(part2(input), 249781879);
}
