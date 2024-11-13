use advent::prelude::*;
use std::iter::{empty, once};

fn default_input() -> &'static str {
    include_input!(2015 / 21)
}

#[derive(Debug, Clone, Copy)]
struct Player {
    damage: i64,
    armor: i64,
    hp: i64,
}

fn wins(mut you: Player, mut boss: Player) -> bool {
    let you_damage = (you.damage - boss.armor).max(1);
    let boss_damage = (boss.damage - you.armor).max(1);

    loop {
        boss.hp -= you_damage;
        if boss.hp <= 0 {
            return true;
        }
        you.hp -= boss_damage;
        if you.hp <= 0 {
            return false;
        }
    }
}

type Item = (i64, i64, i64);
const WEAPONS: &[Item] = &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMOR: &[Item] = &[
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];
const RINGS: &[Item] = &[
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn iter_combos() -> impl Iterator<Item = Item> {
    fn combo(a: (i64, i64, i64), b: (i64, i64, i64)) -> (i64, i64, i64) {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2)
    }

    iproduct!(
        WEAPONS,
        ARMOR,
        empty()
            .chain(RINGS.iter().combinations(2))
            .chain(RINGS.iter().combinations(1))
            .chain(RINGS.iter().combinations(0))
    )
    .map(|(&weapon, &armor, rings)| {
        once(weapon)
            .chain(once(armor))
            .chain(rings.iter().copied().copied())
            .reduce(combo)
            .unwrap()
    })
}

fn part1(input: &str) -> i64 {
    let (hp, damage, armor) = input.nums().tup();
    let boss = Player { damage, armor, hp };

    iter_combos()
        .filter(|(_, damage, armor)| {
            let you = Player {
                damage: *damage,
                armor: *armor,
                hp: 100,
            };

            wins(you, boss)
        })
        .map(|(gold, _, _)| gold)
        .min()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let (hp, damage, armor) = input.nums().tup();
    let boss = Player { damage, armor, hp };

    iter_combos()
        .filter(|(_, damage, armor)| {
            let you = Player {
                damage: *damage,
                armor: *armor,
                hp: 100,
            };

            !wins(you, boss)
        })
        .map(|(gold, _, _)| gold)
        .max()
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert!(wins(
        Player {
            damage: 5,
            armor: 5,
            hp: 8
        },
        Player {
            damage: 7,
            armor: 2,
            hp: 12
        },
    ))
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 111);
    assert_eq!(part2(input), 188);
}
