use advent::prelude::*;

fn default_input() -> (i64, &'static str) {
    (150, include_input!(2015 / 17))
}

fn part1((goal, input): (i64, &str)) -> i64 {
    fn combos(containers: &[i64], liters_left: i64) -> i64 {
        if containers.is_empty() {
            (liters_left == 0) as i64
        } else {
            combos(&containers[1..], liters_left)
                + combos(&containers[1..], liters_left - containers[0])
        }
    }

    let containers = input.nums().collect_vec();
    combos(&containers, goal)
}

fn part2((goal, input): (i64, &str)) -> i64 {
    fn min_containers(containers: &[i64], liters_left: i64, used: i64) -> i64 {
        if containers.is_empty() {
            if liters_left == 0 { used } else { i64::MAX }
        } else {
            let without = min_containers(&containers[1..], liters_left, used);
            let with = min_containers(&containers[1..], liters_left - containers[0], used + 1);
            with.min(without)
        }
    }

    fn combos_using(containers: &[i64], liters_left: i64, used_left: i64) -> i64 {
        if containers.is_empty() {
            (liters_left == 0 && used_left == 0) as i64
        } else {
            combos_using(&containers[1..], liters_left, used_left)
                + combos_using(&containers[1..], liters_left - containers[0], used_left - 1)
        }
    }

    let containers = input.nums().collect_vec();
    let min_containers = min_containers(&containers, goal, 0);
    combos_using(&containers, goal, min_containers)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = (25, "20, 15, 10, 5, 5");
    assert_eq!(part1(input), 4);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 4372);
    assert_eq!(part2(input), 4);
}
