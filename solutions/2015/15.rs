use advent::prelude::*;
use std::ops::{Add, Mul};

fn default_input() -> &'static str {
    include_input!(2015 / 15)
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
}

fn parse(str: &str) -> Vec<Ingredient> {
    str.lines()
        .map(|l| {
            let (a, b, c, d, e) = l.nums().tup();
            Ingredient { a, b, c, d, e }
        })
        .collect()
}

impl Ingredient {
    const ZERO: Ingredient = Ingredient {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
    };

    fn score(self) -> i64 {
        self.a.max(0) * self.b.max(0) * self.c.max(0) * self.d.max(0)
    }
}

impl Mul<i64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, scale: i64) -> Self::Output {
        Ingredient {
            a: self.a * scale,
            b: self.b * scale,
            c: self.c * scale,
            d: self.d * scale,
            e: self.e * scale,
        }
    }
}

impl Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, other: Ingredient) -> Self::Output {
        Ingredient {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
            d: self.d + other.d,
            e: self.e + other.e,
        }
    }
}

fn optimize<S>(base: Ingredient, is: &[Ingredient], tbs_left: i64, score: &S) -> i64
where
    S: Fn(Ingredient) -> i64,
{
    if is.len() == 1 {
        return score(base + is[0] * tbs_left);
    }

    (0..tbs_left)
        .map(|tb| optimize(base + is[0] * tb, &is[1..], tbs_left - tb, score))
        .max()
        .unwrap_or_default()
}

fn part1(input: &str) -> i64 {
    let is = parse(input);
    optimize(Ingredient::ZERO, &is, 100, &Ingredient::score)
}

fn part2(input: &str) -> i64 {
    let is = parse(input);

    optimize(Ingredient::ZERO, &is, 100, &|c| {
        if c.e == 500 {
            c.score()
        } else {
            0
        }
    })
}

fn main() {
    advent::new(2015, 15, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(part1(input), 62842880);
    assert_eq!(part2(input), 57600000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 13882464);
    assert_eq!(part2(input), 11171160);
}
