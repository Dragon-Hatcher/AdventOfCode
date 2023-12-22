use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("256: " / 2017 / 10)
}

fn round(nums: &mut Vec<i64>, lengths: &[i64], pos: &mut usize, skip: &mut usize) {
    for len in lengths {
        let len = *len as usize;

        let end_idx = (*pos + len) % nums.len();
        let mut to_rev = if len != 0 && end_idx <= *pos {
            let mut vec = nums[*pos..].to_vec();
            vec.extend_from_slice(&nums[..end_idx]);
            vec
        } else {
            nums[*pos..*pos + len].to_vec()
        };
        to_rev.reverse();
        if len != 0 && end_idx <= *pos {
            nums.splice(*pos.., to_rev[..(to_rev.len() - end_idx)].iter().copied());
            nums.splice(
                ..end_idx,
                to_rev[(to_rev.len() - end_idx)..].iter().copied(),
            );
        } else {
            nums.splice(*pos..*pos + len, to_rev);
        }

        *pos += len + *skip;
        *pos %= nums.len();
        *skip += 1;
    }
}

fn part1(input: &str) -> i64 {
    let mut lengths = input.nums();
    let size = lengths.nu();
    let lengths = lengths.collect_vec();

    let mut pos = 0;
    let mut skip: usize = 0;
    let mut nums = (0..size).collect_vec();

    round(&mut nums, &lengths, &mut pos, &mut skip);

    nums[0] * nums[1]
}

fn part2(input: &str) -> String {
    let (size, lengths) = input.split_once(':').unwrap();
    let size: i64 = size.parse().unwrap();
    let mut lengths = lengths.trim().bytes().map(|b| b as i64).collect_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut pos = 0;
    let mut skip: usize = 0;
    let mut nums = (0..size).collect_vec();

    for _ in 0..64 {
        round(&mut nums, &lengths, &mut pos, &mut skip);
    }

    nums
        .chunks(16)
        .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .map(|n| format!("{n:02x}"))
        .join("")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "5: 3, 4, 1, 5";
    assert_eq!(part1(input), 12);
    assert_eq!(part2("256: "), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("256: 1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 9656);
    assert_eq!(part2(input), "20b7b54c92bf73cf3e5631458a715149");
}
