use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 20)
}

fn mix(nums: &mut Vec<(usize, i64)>) {
    fn move_left(nums: &mut Vec<(usize, i64)>, i: usize, amount: i64) {
        let amount = amount % (nums.len() as i64 - 1);

        let new_i = if amount < i as i64 {
            i - amount as usize
        } else {
            nums.len() - (amount as usize - i) - 1
        };
        let n = nums.remove(i);
        nums.insert(new_i, n);
    }

    fn move_right(nums: &mut Vec<(usize, i64)>, i: usize, amount: i64) {
        let amount = amount % (nums.len() as i64 - 1);

        let new_i = if i < nums.len() - 1 - amount as usize {
            i + amount as usize
        } else {
            amount as usize - (nums.len() - i - 1)
        };
        let n = nums.remove(i);
        nums.insert(new_i, n);
    }

    for i in 0..nums.len() {
        let from = nums.iter().position(|(j, _)| *j == i).unwrap();
        let (_, n) = nums[from];
        if n >= 0 {
            move_right(nums, from, n);
        } else {
            move_left(nums, from, -n);
        }
    }
}

fn solve(input: &str, decryption_key: i64, mixes: usize) -> i64 {
    let mut nums = input
        .non_empty()
        .map(|l| l.nums().nu() * decryption_key)
        .enumerate()
        .collect_vec();

    for _ in 0..mixes {
        mix(&mut nums);
    }

    let zero_index = nums.iter().position(|(_, n)| *n == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|n| nums[(zero_index + n) % nums.len()].1)
        .sum()
}


fn part1(input: &str) -> i64 {
    solve(input, 1, 1)
}

fn part2(input: &str) -> i64 {
    solve(input, 811589153, 10)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "1
    2
    -3
    3
    -2
    0
    4
    ";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 1623178306);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 7278);
    assert_eq!(part2(input), 14375678667089);
}
