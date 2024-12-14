use advent::prelude::*;
use std::iter::repeat;

fn default_input() -> &'static str {
    include_input!(2024 / 09)
}

fn parse(input: &str) -> (Vec<Option<usize>>, usize) {
    let mut id = 0;
    let mut blocks = vec![];

    for (i, cnt) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        let val = if i % 2 == 0 { Some(id) } else { None };
        id += val.is_some() as usize;
        blocks.extend(repeat(val).take(cnt as usize));
    }

    (blocks, id)
}

fn checksum(blocks: Vec<Option<usize>>) -> i64 {
    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, a)| a.map(|a| i * a))
        .sum::<usize>() as i64
}

fn part1(input: &str) -> i64 {
    let (mut blocks, _) = parse(input);

    let mut file = blocks.len() - 1;
    let mut free = 0;

    loop {
        while blocks[free].is_some() && free < file {
            free += 1;
        }
        while blocks[file].is_none() && free < file {
            file -= 1;
        }
        if free >= file {
            break;
        }
        blocks[free] = blocks[file];
        blocks[file] = None;
    }

    checksum(blocks)
}

fn part2(input: &str) -> i64 {
    let (mut blocks, max_id) = parse(input);
    let mut file_end = blocks.len();

    for id in (0..max_id).rev() {
        while blocks[file_end - 1] != Some(id) {
            file_end -= 1;
        }
        let mut file_start = file_end - 1;
        while file_start > 0 && blocks[file_start - 1] == Some(id) {
            file_start -= 1;
        }
        let len = file_end - file_start;

        let mut gap_start = 0;
        let mut gap_end = 0;
        while gap_end < file_end {
            if gap_end - gap_start == len {
                blocks[gap_start..gap_end].fill(Some(id));
                blocks[file_start..file_end].fill(None);
                break;
            }

            if blocks[gap_end].is_some() {
                gap_start = gap_end + 1;
            }
            gap_end += 1;
        }
    }

    checksum(blocks)
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
