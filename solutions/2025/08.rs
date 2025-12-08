use std::{cmp::Reverse, collections::VecDeque};

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 08)
}

fn part1(input: &str) -> i64 {
    let boxes: Vec<Vec3> = input
        .nums()
        .tuples()
        .map(|(x, y, z)| Vec3::new(x, y, z))
        .collect();

    let mut connected = HashSet::default();

    let mut closest: Vec<VecDeque<(usize, Vec3, f64)>> = boxes
        .iter()
        .map(|&p| {
            boxes
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, a)| p != a)
                .map(|(i, a)| (i, a, (p - a).mag()))
                .sorted_by(|(_, _, d1), (_, _, d2)| f64::total_cmp(d1, d2))
                .collect()
        })
        .collect();

    let mut groups = (0..boxes.len()).collect_vec();

    for _ in 0..1000 {
        loop {
            let mut best_d = f64::MAX;
            let mut best_i = 0;
            let mut best_other = 0;

            for i in 0..boxes.len() {
                if let Some((other_i, _, d)) = closest[i].front() {
                    if *d < best_d {
                        best_d = *d;
                        best_i = i;
                        best_other = *other_i;
                    }
                }
            }

            let (_, c, _) = closest[best_i].pop_front().unwrap();

            if connected.contains(&(boxes[best_i], c)) {
                continue;
            }

            // println!("Merging {:?} and {:?}", boxes[best_i], c);

            connected.insert((boxes[best_i], c));
            connected.insert((c, boxes[best_i]));

            let group_a = groups[best_i];
            let group_b = groups[best_other];

            let merge_to = group_a.min(group_b);
            let merge_from = group_a.max(group_b);

            for g in groups.iter_mut() {
                if *g == merge_from {
                    *g = merge_to;
                }
            }

            break;
        }
    }

    let mut sizes = groups
        .iter()
        .count_occurrences()
        .values()
        .copied()
        .collect_vec();
    sizes.sort_by_key(|&i| Reverse(i));
    // dbg!(&sizes);

    sizes.into_iter().take(3).product()
}

fn part2(input: &str) -> i64 {
    let boxes: Vec<Vec3> = input
        .nums()
        .tuples()
        .map(|(x, y, z)| Vec3::new(x, y, z))
        .collect();

    let mut connected = HashSet::default();

    let mut closest: Vec<VecDeque<(usize, Vec3, f64)>> = boxes
        .iter()
        .map(|&p| {
            boxes
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, a)| p != a)
                .map(|(i, a)| (i, a, (p - a).mag()))
                .sorted_by(|(_, _, d1), (_, _, d2)| f64::total_cmp(d1, d2))
                .collect()
        })
        .collect();

    let mut groups = (0..boxes.len()).collect_vec();

    loop {
        let mut best_d = f64::MAX;
        let mut best_i = 0;
        let mut best_other = 0;

        for i in 0..boxes.len() {
            if let Some((other_i, _, d)) = closest[i].front() {
                if *d < best_d {
                    best_d = *d;
                    best_i = i;
                    best_other = *other_i;
                }
            }
        }

        let (_, c, _) = closest[best_i].pop_front().unwrap();

        if connected.contains(&(boxes[best_i], c)) {
            continue;
        }

        // println!("Merging {:?} and {:?}", boxes[best_i], c);

        connected.insert((boxes[best_i], c));
        connected.insert((c, boxes[best_i]));

        let group_a = groups[best_i];
        let group_b = groups[best_other];

        let merge_to = group_a.min(group_b);
        let merge_from = group_a.max(group_b);

        for g in groups.iter_mut() {
            if *g == merge_from {
                *g = merge_to;
            }
        }

        if groups.iter().all_equal() {
            return c.x * boxes[best_i].x
        }
    }
}

fn main() {
    advent::new(2025, 8, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(part1(input), 40);
    assert_eq!(part2(input), 25272);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
