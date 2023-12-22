use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 22)
}

fn ps(b: (Vector3, Vector3)) -> HashSet<Vector3> {
    let mut seen = HashSet::new();

    for x in b.0.x..=b.1.x {
        for y in b.0.y..=b.1.y {
            for z in b.0.z..=b.1.z {
                seen.insert(Vector3::new(x, y, z));
            }
        }
    }

    seen
}

fn part1(input: &str) -> i64 {
    let mut bricks = input
        .lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) = l.nums().tup();
            (Vector3::new(x1, y1, z1), Vector3::new(x2, y2, z2))
        })
        .collect_vec();

    bricks.sort_by_key(|x| x.0.z);

    let mut top: HashMap<Vector3, usize> = HashMap::new();

    for (idx, mut b) in bricks.iter().copied().enumerate() {
        loop {
            let mut test = b;
            test.0.z -= 1;
            test.1.z -= 1;

            if test.0.z <= 0 || test.1.z <= 0 || ps(test).iter().any(|p| top.contains_key(p)) {
                break;
            }

            b = test;
        }

        for p in ps(b) {
            top.insert(p, idx);
        }
    }

    let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut ground = HashSet::new();

    for (p, group) in &top {
        if p.z == 1 {
            ground.insert(group);
        }

        let down = Vector3::new(p.x, p.y, p.z - 1);
        if down.z <= 0 {
            // eprintln!("{group} => {:?}", supported_by.get(group));
            supported_by.entry(*group).or_default().insert(usize::MAX);
        } else if let Some(dg) = top.get(&down) {
            if dg != group {
                supporting.entry(*dg).or_default().insert(*group);
                supported_by.entry(*group).or_default().insert(*dg);
            }
        }
    }

    // dbg!(&supporting, &supported_by);
    // dbg!(input.lines().count(), bricks.len());

    let mut sum = 0;

    for group in 0..input.lines().count() {
        let supporting = supporting.get(&group);

        if let Some(supporting) = supporting {
            if supporting.iter().any(|g| ground.contains(g)) {
                dbg!(group);
                dbg!(ground.contains(&group));
                dbg!(input.lines().nth(group));
                dbg!(input.lines().nth(72));
                dbg!(
                    supporting,
                    supporting
                        .iter()
                        .map(|g| supported_by.get(g).unwrap())
                        .collect_vec(),
                    supporting.iter().map(|g| ground.contains(g)).collect_vec()
                );
                panic!("What??");
            }

            if supporting
                .iter()
                .any(|g| supported_by.get(g).unwrap().contains(&usize::MAX))
            {
                panic!("What??>");
            }

            if supporting
                .iter()
                .all(|g| supported_by.get(g).unwrap().len() >= 2)
            {
                // eprintln!("{group}");
                sum += 1;
            }
        } else {
            // eprintln!("{group}");
            sum += 1;
        }
    }

    // 545, 614

    sum
}

fn part2(input: &str) -> i64 {
    let mut bricks = input
        .lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) = l.nums().tup();
            (Vector3::new(x1, y1, z1), Vector3::new(x2, y2, z2))
        })
        .collect_vec();

    bricks.sort_by_key(|x| x.0.z);

    let mut top: HashMap<Vector3, usize> = HashMap::new();

    for (idx, mut b) in bricks.iter().copied().enumerate() {
        loop {
            let mut test = b;
            test.0.z -= 1;
            test.1.z -= 1;

            if test.0.z <= 0 || test.1.z <= 0 || ps(test).iter().any(|p| top.contains_key(p)) {
                break;
            }

            b = test;
        }

        for p in ps(b) {
            top.insert(p, idx);
        }
    }

    let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut ground = HashSet::new();

    for (p, group) in &top {
        if p.z == 1 {
            ground.insert(group);
        }

        let down = Vector3::new(p.x, p.y, p.z - 1);
        if down.z <= 0 {
            // eprintln!("{group} => {:?}", supported_by.get(group));
            supported_by.entry(*group).or_default().insert(usize::MAX);
        } else if let Some(dg) = top.get(&down) {
            if dg != group {
                supporting.entry(*dg).or_default().insert(*group);
                supported_by.entry(*group).or_default().insert(*dg);
            }
        }
    }

    // dbg!(&supporting, &supported_by);
    // dbg!(input.lines().count(), bricks.len());

    let mut sum = 0;

    for group in 0..input.lines().count() {
        let mut fallen = HashSet::new();
        fallen.insert(group);

        loop {
            let next: HashSet<usize> = fallen
                .iter()
                .copied()
                .flat_map(|g| supporting.get(&g).cloned().unwrap_or_default())
                .filter(|g| supported_by.get(g).unwrap().difference(&fallen).count() == 0)
                .collect();
                // .collect();

            // dbg!(group, &next);

            if next.difference(&fallen).count() == 0 {
                sum += fallen.len() - 1;
                break;
            }

            fallen.extend(next);
        }

    }

    // 545, 614

    sum as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 7);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 492);
    assert_eq!(part2(input), 86556);
}
