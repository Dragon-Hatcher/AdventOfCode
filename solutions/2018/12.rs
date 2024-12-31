use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 12)
}

fn parse(input: &str) -> (HashSet<i64>, HashSet<(bool, bool, bool, bool, bool)>) {
    let (initial, replacements) = input.sections().tup();

    let initial = initial
        .strip_prefix("initial state: ")
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i as i64))
        .collect();

    let rules = replacements
        .lines()
        .filter_map(|l| {
            let (pattern, out) = l.split_once(" => ").unwrap();
            let pattern = pattern.chars().map(|c| c == '#').tup();

            (out == "#").then_some(pattern)
        })
        .collect();

    (initial, rules)
}

fn solve(input: &str, mut iters: i64) -> i64 {
    let (mut active, replacements) = parse(input);
    let mut offset = 0;

    fn normalize(active: &mut HashSet<i64>) -> i64 {
        let min = *active.iter().min().unwrap();
        *active = active.iter().map(|x| x - min).collect();
        min
    }

    while iters > 0 {
        let to_consider = active
            .iter()
            .flat_map(|&i| [i - 2, i - 1, i, i + 1, i + 2])
            .unique();

        let mut new_active = to_consider
            .filter(|i| {
                let curr = (
                    active.contains(&(i - 2)),
                    active.contains(&(i - 1)),
                    active.contains(&(i)),
                    active.contains(&(i + 1)),
                    active.contains(&(i + 2)),
                );

                replacements.contains(&curr)
            })
            .collect();

        offset += normalize(&mut new_active);
        iters -= 1;

        if active == new_active {
            offset += iters;
            iters -= iters;
        }

        active = new_active;
    }

    active.into_iter().map(|i| i + offset).sum()
}

fn part1(input: &str) -> i64 {
    solve(input, 20)
}

fn part2(input: &str) -> i64 {
    solve(input, 50000000000)
}

fn main() {
    advent::new(2018, 12, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
    assert_eq!(part1(input), 325);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2349);
    assert_eq!(part2(input), 2100000001168);
}
