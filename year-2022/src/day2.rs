use crate::standard_parsers::{AocParsed, IntoTup};

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

///
/// --- Day 2: Rock Paper Scissors ---
///
/// The Elves begin to set up camp on the beach. To decide whose tent gets to be
/// closest to the snack storage, a giant [Rock Paper Scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors)
/// tournament is already in progress.
///
/// Rock Paper Scissors is a game between two players. Each game contains many rounds;
/// in each round, the players each simultaneously choose one of Rock, Paper, or
/// Scissors using a hand shape. Then, a winner for that round is selected: Rock
/// defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players
/// choose the same shape, the round instead ends in a draw.
///
/// Appreciative of your help yesterday, one Elf gives you an *encrypted strategy
/// guide* (your puzzle input) that they say will be sure to help you win. "The first
/// column is what your opponent is going to play: `A` for Rock, `B` for Paper, and
/// `C` for Scissors. The second column--" Suddenly, the Elf is called away to help
/// with someone's tent.
///
/// The second column, you reason, must be what you should play in response: `X`
/// for Rock, `Y` for Paper, and `Z` for Scissors. Winning every time would be suspicious,
/// so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score. Your
/// *total score* is the sum of your scores for each round. The score for a single
/// round is the score for the *shape you selected* (1 for Rock, 2 for Paper, and
/// 3 for Scissors) plus the score for the *outcome of the round* (0 if you lost,
/// 3 if the round was a draw, and 6 if you won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you should
/// calculate the score you would get if you were to follow the strategy guide.
///
/// For example, suppose you were given the following strategy guide:
///
/// ```
/// A Y
/// B X
/// C Z
///
/// ```
///
/// This strategy guide predicts and recommends the following:
///
/// * In the first round, your opponent will choose Rock (`A`), and you should choose
/// Paper (`Y`). This ends in a win for you with a score of *8* (2 because you chose
/// Paper + 6 because you won).
/// * In the second round, your opponent will choose Paper (`B`), and you should
/// choose Rock (`X`). This ends in a loss for you with a score of *1* (1 + 0).
/// * The third round is a draw with both players choosing Scissors, giving you a
/// score of 3 + 3 = *6*.
///
/// In this example, if you were to follow the strategy guide, you would get a total
/// score of `*15*` (8 + 1 + 6).
///
/// *What would your total score be if everything goes exactly according to your
/// strategy guide?*
///
pub fn part1(input: &str) -> i64 {
    input
        .non_empty()
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

///
/// --- Part Two ---
///
/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
/// the second column says how the round needs to end: `X` means you need to lose,
/// `Y` means you need to end the round in a draw, and `Z` means you need to win.
/// Good luck!"
///
/// The total score is still calculated in the same way, but now you need to figure
/// out what shape to choose so the round ends as indicated. The example above now
/// goes like this:
///
/// * In the first round, your opponent will choose Rock (`A`), and you need the
/// round to end in a draw (`Y`), so you also choose Rock. This gives you a score
/// of 1 + 3 = *4*.
/// * In the second round, your opponent will choose Paper (`B`), and you choose
/// Rock so you lose (`X`) with a score of 1 + 0 = *1*.
/// * In the third round, you will defeat your opponent's Scissors with Rock for
/// a score of 1 + 6 = *7*.
///
/// Now that you're correctly decrypting the ultra top secret strategy guide, you
/// would get a total score of `*12*`.
///
/// Following the Elf's instructions for the second column, *what would your total
/// score be if everything goes exactly according to your strategy guide?*
///
pub fn part2(input: &str) -> i64 {
    input
        .non_empty()
        .map(|l| {
            let (opp, _, outcome) = l.chars().tup();
            let opp = move_from_char(opp);
            let outcome = outcome_from_char(outcome);
            let me = move_for_outcome(opp, outcome);

            me.value() + outcome.value()
        })
        .sum()
}

const PART1_EX_ANSWER: &str = "15";
const PART1_ANSWER: &str = "13484";
const PART2_EX_ANSWER: &str = "12";
const PART2_ANSWER: &str = "13433";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
