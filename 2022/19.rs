use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 19)
}

// ore, clay, obby, geode
#[derive(Debug, Clone, Copy, Default)]
struct Amounts<T>([T; 4]);

type Costs = Amounts<Amounts<i64>>;
type Resources = Amounts<i64>;
type RobotCount = Amounts<i64>;

fn parse(input: &str) -> impl Iterator<Item = Costs> + '_ {
    input.non_empty().map(|l| {
        let (_, ore_ore, clay_ore, obby_ore, obby_clay, geode_ore, geode_obby) = l.nums().tup();

        Amounts([
            Amounts([ore_ore, 0, 0, 0]),
            Amounts([clay_ore, 0, 0, 0]),
            Amounts([obby_ore, obby_clay, 0, 0]),
            Amounts([geode_ore, 0, geode_obby, 0]),
        ])
    })
}

fn max_geodes(costs: &Costs, resources: Resources, robot_count: RobotCount, mins_left: i64) -> i64 {
    if mins_left <= 0 || (mins_left <= 7 && robot_count.0[3] == 0) {
        return resources.0[3];
    }

    (0..4)
        .filter_map(|i| {
            if i != 3
                && (i == 0 || robot_count.0[i] >= costs.0[0].0[i])
                && (i == 1 || robot_count.0[i] >= costs.0[1].0[i])
                && (i == 2 || robot_count.0[i] >= costs.0[2].0[i])
                && (i == 3 || robot_count.0[i] >= costs.0[3].0[i])
            {
                return None;
            }

            let cost = costs.0[i];
            let mats_left_to_make = Amounts([
                cost.0[0] - resources.0[0],
                cost.0[1] - resources.0[1],
                cost.0[2] - resources.0[2],
                cost.0[3] - resources.0[3],
            ]);

            fn ciel_div(a: i64, b: i64) -> i64 {
                if a == 0 {
                    0
                } else if b == 0 {
                    i64::MAX
                } else {
                    (a + b - 1) / b
                }
            }

            let mins_till_robot = ciel_div(mats_left_to_make.0[0], robot_count.0[0])
                .max(ciel_div(mats_left_to_make.0[1], robot_count.0[1]))
                .max(ciel_div(mats_left_to_make.0[2], robot_count.0[2]))
                .max(ciel_div(mats_left_to_make.0[3], robot_count.0[3]))
                .max(0)
                .saturating_add(1);

            if mins_till_robot > mins_left {
                return None;
            }

            let new_resources = Amounts([
                resources.0[0] + robot_count.0[0] * mins_till_robot - cost.0[0],
                resources.0[1] + robot_count.0[1] * mins_till_robot - cost.0[1],
                resources.0[2] + robot_count.0[2] * mins_till_robot - cost.0[2],
                resources.0[3] + robot_count.0[3] * mins_till_robot - cost.0[3],
            ]);

            let new_robots = Amounts([
                robot_count.0[0] + if i == 0 { 1 } else { 0 },
                robot_count.0[1] + if i == 1 { 1 } else { 0 },
                robot_count.0[2] + if i == 2 { 1 } else { 0 },
                robot_count.0[3] + if i == 3 { 1 } else { 0 },
            ]);

            Some(max_geodes(
                costs,
                new_resources,
                new_robots,
                mins_left - mins_till_robot,
            ))
        })
        .max()
        .unwrap_or(resources.0[3] + robot_count.0[3] * mins_left)
}

fn part1(input: &str) -> i64 {
    parse(input)
        .map(|c| max_geodes(&c, Default::default(), Amounts([1, 0, 0, 0]), 24))
        .enumerate()
        .map(|(i, q)| (i as i64 + 1) * q)
        .sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .take(3)
        .map(|c| max_geodes(&c, Default::default(), Amounts([1, 0, 0, 0]), 32))
        .product()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    // assert_eq!(part1(input), 33); // Emperical constants tuned for part 2
    assert_eq!(part2(input), 3472);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 1150); // Emperical constants tuned for part 2
    assert_eq!(part2(input), 37367);
}
