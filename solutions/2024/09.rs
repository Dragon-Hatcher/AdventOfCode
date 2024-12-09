use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 09)
}

fn part1(input: &str) -> i64 {
    let mut nums = input.trim().chars().map(|c| c.to_digit(10).unwrap());
    let mut blocks = vec![];

    let mut id = 0;
    while let Some(file) = nums.next() {
        for _ in 0..file {
            blocks.push(Some(id));
        }
        id += 1;
        if let Some(space) = nums.next() {
            for _ in 0..space {
                blocks.push(None);
            }
        }
    }

    let mut back = blocks.len() - 1;
    let mut free = 0;

    while free < back {
        while blocks[free].is_some() && free < back {
            free += 1;
        }
        if free >= back {
            break;
        }
        blocks[free] = blocks[back];
        blocks[back] = None;
        while blocks[back].is_none() && back > free {
            back -= 1;
        }
    }

    // dbg!(blocks);
    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, a)| a.map(|a| i * a))
        .sum::<usize>() as i64
}

fn part2(input: &str) -> i64 {
    let mut nums = input.trim().chars().map(|c| c.to_digit(10).unwrap());
    let mut blocks = vec![];

    let mut id = 0;
    while let Some(file) = nums.next() {
        for _ in 0..file {
            blocks.push(Some(id));
        }
        id += 1;
        if let Some(space) = nums.next() {
            for _ in 0..space {
                blocks.push(None);
            }
        }
    }

    for id in (0..id).rev() {
        let mut start = 0;
        while blocks[start] != Some(id) {
            start += 1;
        }
        let mut end = start;
        while end < blocks.len() && blocks[end] == Some(id) {
            end += 1;
        }
        let len = end - start;

        let mut s_start = 0;
        while s_start + len < end {
            s_start += 1;

            if s_start + len < end && blocks[s_start..s_start + len].iter().all(|b| b.is_none()) {
                blocks[s_start..s_start + len].fill(Some(id));
                blocks[start..end].fill(None);
                break;
            }
        }
    }

    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, a)| a.map(|a| i * a))
        .sum::<usize>() as i64
}

fn main() {
    advent::new(2024, 9, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "2333133121414131402";
    assert_eq!(part1(input), 1928);
    assert_eq!(part2(input), 2858);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6349606724455);
    assert_eq!(part2(input), 6376648986651);
}
