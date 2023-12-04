use advent::prelude::*;
use memoize::memoize;

fn default_input() -> String {
    format!(
        "{} player hp = 50, player_mana = 500",
        include_input!(2015 / 22)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GameState {
    player_hp: u8,
    boss_hp: u8,
    mana: u16,
    armor_cooldown: u8,
    poison_cooldown: u8,
    recharge_cooldown: u8,
    player_turn: bool,
}

#[memoize]
fn solve(state: GameState, boss_damage: u8, attrition: u8) -> i64 {
    let mut state = state;

    if state.player_turn {
        state.player_hp = state.player_hp.saturating_sub(attrition);
    }
    if state.player_hp == 0 {
        return i64::MAX;
    }

    let poison_damage = if state.poison_cooldown > 0 { 3 } else { 0 };
    let player_armor = if state.armor_cooldown > 0 { 7 } else { 0 };
    let mana_recharge = if state.recharge_cooldown > 0 { 101 } else { 0 };

    let player_turn = state.player_turn;

    state.armor_cooldown = state.armor_cooldown.saturating_sub(1);
    state.poison_cooldown = state.poison_cooldown.saturating_sub(1);
    state.recharge_cooldown = state.recharge_cooldown.saturating_sub(1);

    state.player_turn = !state.player_turn;

    state.boss_hp = state.boss_hp.saturating_sub(poison_damage);
    state.mana = state.mana.saturating_add(mana_recharge);

    if state.boss_hp == 0 {
        return 0;
    }

    if player_turn {
        let mut best = i64::MAX;

        if state.mana >= 53 {
            let mut new_s = state;
            new_s.boss_hp = new_s.boss_hp.saturating_sub(4);
            new_s.mana = new_s.mana.saturating_sub(53);
            best = best.min(solve(new_s, boss_damage, attrition).saturating_add(53));
        }

        if state.mana >= 73 {
            let mut new_s = state;
            new_s.boss_hp = new_s.boss_hp.saturating_sub(2);
            new_s.player_hp = new_s.player_hp.saturating_add(2);
            new_s.mana = new_s.mana.saturating_sub(73);
            best = best.min(solve(new_s, boss_damage, attrition).saturating_add(73));
        }

        if state.mana >= 113 && state.armor_cooldown == 0 {
            let mut new_s = state;
            new_s.armor_cooldown = 6;
            new_s.mana = new_s.mana.saturating_sub(113);
            best = best.min(solve(new_s, boss_damage, attrition).saturating_add(113));
        }

        if state.mana >= 173 && state.poison_cooldown == 0 {
            let mut new_s = state;
            new_s.poison_cooldown = 6;
            new_s.mana = new_s.mana.saturating_sub(173);
            best = best.min(solve(new_s, boss_damage, attrition).saturating_add(173));
        }

        if state.mana >= 229 && state.recharge_cooldown == 0 {
            let mut new_s = state;
            new_s.recharge_cooldown = 5;
            new_s.mana = new_s.mana.saturating_sub(229);
            best = best.min(solve(new_s, boss_damage, attrition).saturating_add(229));
        }

        best
    } else {
        let damage = (boss_damage - player_armor).max(1);
        state.player_hp = state.player_hp.saturating_sub(damage);

        if state.player_hp == 0 {
            return i64::MAX;
        }

        solve(state, boss_damage, attrition)
    }
}

fn part1(input: String) -> i64 {
    let (boss_hp, boss_damage, player_hp, mana) = input.nums().tup();

    solve(
        GameState {
            player_hp: player_hp as u8,
            boss_hp: boss_hp as u8,
            mana: mana as u16,
            armor_cooldown: 0,
            poison_cooldown: 0,
            recharge_cooldown: 0,
            player_turn: true,
        },
        boss_damage as u8,
        0
    )
}

fn part2(input: String) -> i64 {
    let (boss_hp, boss_damage, player_hp, mana) = input.nums().tup();

    solve(
        GameState {
            player_hp: player_hp as u8,
            boss_hp: boss_hp as u8,
            mana: mana as u16,
            armor_cooldown: 0,
            poison_cooldown: 0,
            recharge_cooldown: 0,
            player_turn: true,
        },
        boss_damage as u8,
        1
    )
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    assert_eq!(part1("boss_hp=13 damage=8 player_hp=10 mana=250".to_owned()), 226);
    assert_eq!(part1("boss_hp=14 damage=8 player_hp=10 mana=250".to_owned()), 641);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 900);
    assert_eq!(part2(input), 1216);
}
