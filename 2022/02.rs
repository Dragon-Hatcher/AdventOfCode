use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 02)
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Outcome {
    fn value(self) -> i64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

fn move_from_char(char: char) -> Move {
    match char {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("bad letter"),
    }
}

fn outcome(me: Move, opp: Move) -> Outcome {
    match (me, opp) {
        (Move::Rock, Move::Rock) => Outcome::Tie,
        (Move::Rock, Move::Paper) => Outcome::Lose,
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Paper, Move::Paper) => Outcome::Tie,
        (Move::Paper, Move::Scissors) => Outcome::Lose,
        (Move::Scissors, Move::Rock) => Outcome::Lose,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Scissors, Move::Scissors) => Outcome::Tie,
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (opp, _, me) = l.chars().tup();
            let opp = move_from_char(opp);
            let me = move_from_char(me);
            let outcome = outcome(me, opp);

            me.value() + outcome.value()
        })
        .sum()
}

fn outcome_from_char(char: char) -> Outcome {
    match char {
        'X' => Outcome::Lose,
        'Y' => Outcome::Tie,
        _ => Outcome::Win,
    }
}

fn move_for_outcome(opp: Move, outcome: Outcome) -> Move {
    match (opp, outcome) {
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Rock, Outcome::Tie) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Paper, Outcome::Tie) => Move::Paper,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
        (Move::Scissors, Outcome::Tie) => Move::Scissors,
        (Move::Scissors, Outcome::Win) => Move::Rock,
    }
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (opp, _, outcome) = l.chars().tup();
            let opp = move_from_char(opp);
            let outcome = outcome_from_char(outcome);
            let me = move_for_outcome(opp, outcome);

            me.value() + outcome.value()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "A Y
B X
C Z";

    assert_eq!(part1(input), 15);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 13484);
    assert_eq!(part2(input), 13433);
}
