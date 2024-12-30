use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 12)
}

fn pretty(active: &HashSet<i64>) {
    let min = *active.iter().min().unwrap();

    let grid = Grid::new_with(200, 1, |p| active.contains(&(p.x + min)));
    println!("{}", grid.pretty());
}

fn part1(input: &str) -> i64 {
    let (initial, rules) = input.sections().tup();

    let initial = initial.strip_prefix("initial state: ").unwrap();
    let mut active: HashSet<i64> = initial
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i as i64))
        .collect();

    let rules: HashSet<_> = rules
        .lines()
        .filter_map(|l| {
            let (pattern, out) = l.split_once(" => ").unwrap();

            let pattern: (_, _, _, _, _) = pattern.chars().map(|c| c == '#').tup();
            let out = out == "#";

            out.then_some(pattern)
        })
        .collect();

    for _ in 0..300 {
        pretty(&active);

        let to_consider = active
            .iter()
            .flat_map(|&i| [i - 2, i - 1, i, i + 1, i + 2])
            .unique();

        let mut new_active = HashSet::default();

        for i in to_consider {
            let curr = (
                active.contains(&(i - 2)),
                active.contains(&(i - 1)),
                active.contains(&(i)),
                active.contains(&(i + 1)),
                active.contains(&(i + 2)),
            );

            if rules.contains(&curr) {
                new_active.insert(i);
            }
        }

        active = new_active;
    }

    active.into_iter().sum()
}

fn normalize(active: &mut HashSet<i64>) -> i64 {
    let min = *active.iter().min().unwrap();
    let new = active.iter().map(|x| x - min).collect();
    *active = new;

    min
}

fn part2(input: &str) -> i64 {
    let (initial, rules) = input.sections().tup();

    let initial = initial.strip_prefix("initial state: ").unwrap();
    let mut active: HashSet<i64> = initial
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i as i64))
        .collect();
    let mut previous = HashSet::default();
    let mut offset = 0;
    let mut iters_left = 50000000000;

    let rules: HashSet<_> = rules
        .lines()
        .filter_map(|l| {
            let (pattern, out) = l.split_once(" => ").unwrap();

            let pattern: (_, _, _, _, _) = pattern.chars().map(|c| c == '#').tup();
            let out = out == "#";

            out.then_some(pattern)
        })
        .collect();

    while previous != active {
        let to_consider = active
            .iter()
            .flat_map(|&i| [i - 2, i - 1, i, i + 1, i + 2])
            .unique();

        let mut new_active = HashSet::default();

        for i in to_consider {
            let curr = (
                active.contains(&(i - 2)),
                active.contains(&(i - 1)),
                active.contains(&(i)),
                active.contains(&(i + 1)),
                active.contains(&(i + 2)),
            );

            if rules.contains(&curr) {
                new_active.insert(i);
            }
        }

        offset += normalize(&mut new_active);

        iters_left -= 1;
        previous = active;
        active = new_active;
    }

    active.into_iter().map(|i| i + offset + iters_left).sum()
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
    assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
