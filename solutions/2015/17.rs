use advent::prelude::*;

fn default_input() -> (i64, &'static str) {
    (150, include_input!(2015 / 17))
}

fn part1((target, input): (i64, &str)) -> i64 {
    fn count_combos(containers: &[i64], target: i64) -> i64 {
        if target < 0 {
            return 0;
        }

        if containers.is_empty() {
            return (target == 0) as i64;
        }

        let rest = &containers[1..];
        count_combos(rest, target) + count_combos(rest, target - containers[0])
    }

    let containers = input.nums().collect_vec();
    count_combos(&containers, target)
}

fn part2((target, input): (i64, &str)) -> i64 {
    fn min_containers(containers: &[i64], target: i64) -> i64 {
        if target < 0 {
            return i64::MAX / 2;
        }

        if containers.is_empty() {
            return if target == 0 { 0 } else { i64::MAX / 2 };
        }

        let rest = &containers[1..];
        let without = min_containers(rest, target);
        let with = min_containers(rest, target - containers[0]) + 1;

        with.min(without)
    }

    fn count_combos_using(containers: &[i64], target: i64, left: i64) -> i64 {
        if target < 0 || left < 0 {
            return 0;
        }

        if containers.is_empty() {
            return (target == 0 && left == 0) as i64;
        }

        let rest = &containers[1..];
        count_combos_using(rest, target, left)
            + count_combos_using(rest, target - containers[0], left - 1)
    }

    let containers = input.nums().collect_vec();
    let min = min_containers(&containers, target);
    count_combos_using(&containers, target, min)
}

fn main() {
    advent::new(2015, 17, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
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
