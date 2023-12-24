use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 14)
}

fn hash(input: &str) -> u128 {
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

    let mut lengths = input.bytes().map(|b| b as i64).collect_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut pos = 0;
    let mut skip: usize = 0;
    let mut nums = (0..256).collect_vec();

    for _ in 0..64 {
        round(&mut nums, &lengths, &mut pos, &mut skip);
    }

    let nums = nums
        .chunks(16)
        .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .map(|n| n as u8)
        .collect_vec();

    u128::from_be_bytes(nums.as_slice().try_into().unwrap())
}

fn part1(input: &str) -> i64 {
    let key = input.trim();

    (0..128)
        .map(|n| format!("{key}-{n}"))
        .map(|k| hash(&k).count_ones())
        .sum::<u32>() as i64
}

fn part2(input: &str) -> i64 {
    let key = input.trim();
    let mut grid = Grid::new_with(128, 128, |_| false);

    for row in 0..128 {
        let hash = hash(&format!("{key}-{row}"));
        for col in 0..128 {
            let p = Vector2::new(col, row);
            grid[p] = (hash & (1 << col)) != 0
        }
    }

    let mut seen: HashSet<Vector2> = HashSet::new();
    let mut group_cnt = 0;

    for p in grid.points() {
        if grid[p] && !seen.contains(&p) {
            group_cnt += 1;

            let mut edge = HashSet::new();
            edge.insert(p);

            while !edge.is_empty() {
                edge = edge
                    .iter()
                    .flat_map(|p| p.neighbors4())
                    .filter(|p| grid.in_bounds(*p) && grid[*p] && !seen.contains(p))
                    .collect();
                seen.extend(&edge);
            }
        }
    }

    group_cnt
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "flqrgnkx";
    assert_eq!(part1(input), 8108);
    assert_eq!(part2(input), 1242);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 8216);
    assert_eq!(part2(input), 1139);
}
