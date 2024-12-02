use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 02)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| {
            l.nums().tuple_windows().all(|(a, b)| a < b && (a - b).abs() >= 1 && (a - b).abs() <= 3) ||
            l.nums().tuple_windows().all(|(a, b)| a > b && (a - b).abs() >= 1 && (a - b).abs() <= 3)
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|l| {
            let nums = l.nums().collect_vec();

            if l.nums().tuple_windows().all(|(a, b)| a < b && (a - b).abs() >= 1 && (a - b).abs() <= 3) ||
            l.nums().tuple_windows().all(|(a, b)| a > b && (a - b).abs() >= 1 && (a - b).abs() <= 3) {
                return true
            }

            for i in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(i);

                if nums.clone().into_iter().tuple_windows().all(|(a, b)| a < b && (a - b).abs() >= 1 && (a - b).abs() <= 3) ||
                nums.clone().into_iter().tuple_windows().all(|(a, b)| a > b && (a - b).abs() >= 1 && (a - b).abs() <= 3) {
                return true
            }
            }

            false
        })
        .count() as i64
}

fn main() {
    advent::new(2024, 2, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 411);
    assert_eq!(part2(input), 465);
}
