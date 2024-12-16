use std::iter::StepBy;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 16)
}

fn find_cost<F>(grid: &Grid<bool>, start: Vec2, start_dir: Direction, end: F) -> i64
where
    F: Fn(Vec2, Direction) -> bool,
{
    let mut seen = HashMap::default();
    let mut poses = MinBinaryHeap::new_min();
    poses.push((0, start, start_dir));
    seen.insert((start, start_dir), 0);

    while let Some((len, pos, dir)) = poses.pop() {
        if end(pos, dir) {
            return len;
        }

        if !grid[pos + dir.vector()] {
            let key = (pos + dir.vector(), dir);
            let new = (len + 1, pos + dir.vector(), dir);

            if seen.get(&key).is_none() || seen[&key] > len + 1 {
                seen.insert(key, len + 1);
                poses.push(new);
            }
        }

        {
            let key = (pos, dir.turn_left());
            let new = (len + 1000, pos, dir.turn_left());

            if seen.get(&key).is_none() || seen[&key] > len + 1000 {
                seen.insert(key, len + 1000);
                poses.push(new);
            }
        }

        {
            let key = (pos, dir.turn_right());
            let new = (len + 1000, pos, dir.turn_right());

            if seen.get(&key).is_none() || seen[&key] > len + 1000 {
                seen.insert(key, len + 1000);
                poses.push(new);
            }
        }
    }

    unreachable!()
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c == '#');
    let pos = Grid::new_by_char(input, |c| c);

    let start = pos.points().find(|p| pos[*p] == 'S').unwrap();
    let end = pos.points().find(|p| pos[*p] == 'E').unwrap();

    find_cost(&grid, start, Direction::Right, |p, _| p == end)
}

fn part2(input: &str) -> i64 {
    let best = part1(input);

    let grid = Grid::new_by_char(input, |c| c == '#');
    let pos = Grid::new_by_char(input, |c| c);

    let start = pos.points().find(|p| pos[*p] == 'S').unwrap();
    let end = pos.points().find(|p| pos[*p] == 'E').unwrap();

    let mut included = HashSet::default();

    for (i, p) in grid.points().enumerate() {
        if grid[p] {
            continue;
        }

        println!("{i} / {}", grid.width() * grid.height());

        for d in Direction::ALL {
            let start_to_here =
                find_cost(&grid, start, Direction::Right, |ep, ed| ep == p && ed == d);
            let here_to_end = find_cost(&grid, p, d, |ep, _| ep == end);

            if start_to_here + here_to_end <= best {
                included.insert(p);
            }
        }
    }

    included.len() as i64

    // let mut on_best: HashSet<Vec2> = HashSet::default();

    // let critical_points: HashSet<_> = grid
    //     .points()
    //     .filter(|&p| {
    //         if p == start || p == end {
    //             return true;
    //         }

    //         if grid[p] {
    //             return false;
    //         }

    //         let cnt = (grid[p + Vec2::E1] == false) as i64
    //             + (grid[p - Vec2::E1] == false) as i64
    //             + (grid[p + Vec2::E2] == false) as i64
    //             + (grid[p - Vec2::E2] == false) as i64;

    //         if p == v2(2, 13) {
    //             dbg!(cnt);
    //         }

    //         cnt > 2
    //     })
    //     .collect();

    // let mut map: HashMap<(Vec2, Direction), HashMap<(Vec2, Direction), i64>> = HashMap::default();

    // for p in critical_points.iter() {
    //     for d in Direction::ALL {
    //         let mut seen = HashMap::default();
    //         let mut poses = MinBinaryHeap::new_min();
    //         poses.push((0, *p, d));
    //         seen.insert((*p, d), 0);

    //         let entry = map.entry((*p, d)).or_default();

    //         while let Some((len, pos, dir)) = poses.pop() {
    //             if pos != *p && critical_points.contains(&pos) && !entry.contains_key(&(pos, dir)) {
    //                 entry.insert((pos, dir), len);
    //             }

    //             if (!critical_points.contains(&pos) || pos == *p) && !grid[pos + dir.vector()] {
    //                 let key = (pos + dir.vector(), dir);
    //                 let new = (len + 1, pos + dir.vector(), dir);

    //                 if seen.get(&key).is_none() || seen[&key] > len + 1 {
    //                     seen.insert(key, len + 1);
    //                     poses.push(new);
    //                 }
    //             }

    //             {
    //                 let key = (pos, dir.turn_left());
    //                 let new = (len + 1000, pos, dir.turn_left());

    //                 if seen.get(&key).is_none() || seen[&key] > len + 1000 {
    //                     seen.insert(key, len + 1000);
    //                     poses.push(new);
    //                 }
    //             }

    //             {
    //                 let key = (pos, dir.turn_right());
    //                 let new = (len + 1000, pos, dir.turn_right());

    //                 if seen.get(&key).is_none() || seen[&key] > len + 1000 {
    //                     seen.insert(key, len + 1000);
    //                     poses.push(new);
    //                 }
    //             }
    //         }
    //     }
    // }

    // dbg!(&map[&(start, Direction::Right)].contains_key(&(v2(2, 13), Direction::Right)));

    // fn solve(
    //     map: &HashMap<(Vec2, Direction), HashMap<(Vec2, Direction), i64>>,
    //     at: Vec2,
    //     dir: Direction,
    //     goal: Vec2,
    //     best: i64,
    //     len_so_far: i64,
    //     path: &mut Vec<(Vec2, Direction)>,
    //     on_best: &mut HashSet<Vec2>,
    // ) {
    //     if len_so_far < 90000 {
    //         println!("{:?}", path);
    //     }

    //     if at == goal {
    //         if len_so_far == best {
    //             on_best.extend(path.into_iter().map(|(p, _)| *p));
    //         }

    //         return;
    //     }

    //     if len_so_far > best {
    //         return;
    //     }

    //     let moves = &map[&(at, dir)];

    //     for ((new_at, new_dir), cost) in moves {
    //         if path.contains(&(*new_at, *new_dir)) {
    //             continue;
    //         }

    //         path.push((*new_at, *new_dir));
    //         solve(
    //             map,
    //             *new_at,
    //             *new_dir,
    //             goal,
    //             best,
    //             len_so_far + cost,
    //             path,
    //             on_best,
    //         );
    //         path.pop();
    //     }

    // if !grid[at + dir.vector()] {
    //     let key = (at + dir.vector(), dir);
    //     // let new = (len + 1, pos + dir.vector(), dir);

    //     if !path.contains(&key) {
    //         path.push((at + dir.vector(), dir));
    //         solve(
    //             grid,
    //             at + dir.vector(),
    //             dir,
    //             goal,
    //             best,
    //             len_so_far + 1,
    //             path,
    //             on_best,
    //         );
    //         path.pop();
    //     }
    // }

    // {
    //     let key = (at, dir.turn_left());

    //     if !path.contains(&key) {
    //         path.push((at, dir.turn_left()));
    //         solve(
    //             grid,
    //             at,
    //             dir.turn_left(),
    //             goal,
    //             best,
    //             len_so_far + 1000,
    //             path,
    //             on_best,
    //         );
    //         path.pop();
    //     }
    // }

    // {
    //     let key = (at, dir.turn_right());

    //     if !path.contains(&key) {
    //         path.push((at, dir.turn_right()));
    //         solve(
    //             grid,
    //             at,
    //             dir.turn_right(),
    //             goal,
    //             best,
    //             len_so_far + 1000,
    //             path,
    //             on_best,
    //         );
    //         path.pop();
    //     }
    // }
    // }

    // let mut v = vec![(start, Direction::Right)];
    // solve(
    //     &map,
    //     start,
    //     Direction::Right,
    //     end,
    //     best,
    //     0,
    //     &mut v,
    //     &mut on_best,
    // );

    // while let Some((len, pos, dir, path)) = poses.pop() {
    //     if pos == end {
    //         if len == best {
    //             on_best.extend(path.into_iter().map(|(p, _)| p));
    //         }

    //         continue;
    //     }

    //     if len > best {
    //         continue;
    //     }

    //     if !grid[pos + dir.vector()] {
    //         let key = (pos + dir.vector(), dir);
    //         // let new = (len + 1, pos + dir.vector(), dir);

    //         if !path.contains(&key) {
    //             seen.insert(key, len + 1);
    //             let mut path = path.clone();
    //             path.push(key);
    //             poses.push((len + 1, pos + dir.vector(), dir, path));
    //         }
    //     }

    //     {
    //         let key = (pos, dir.turn_left());
    //         // let new = (len + 1000, pos, dir.turn_left());

    //         if !path.contains(&key) {
    //             seen.insert(key, len + 1000);
    //             let mut path = path.clone();
    //             path.push(key);
    //             poses.push((len + 1000, pos, dir.turn_left(), path));
    //         }
    //     }

    //     {
    //         let key = (pos, dir.turn_right());
    //         // let new = (len + 1000, pos, dir.turn_right());

    //         if !path.contains(&key) {
    //             seen.insert(key, len + 1000);
    //             let mut path = path.clone();
    //             path.push(key);
    //             poses.push((len + 1000, pos, dir.turn_right(), path));
    //         }
    //     }
    // }

    // on_best.len() as i64
}

fn main() {
    advent::new(2024, 16, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part1(input), 7036);
    assert_eq!(part2(input), 45);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
