use advent::prelude::*;

fn parse(str: &str) -> FxHashSet<Vector3> {
    str.lines()
        .map(|l| {
            let (x, y, z) = l.nums().tup();
            Vector3::new(x, y, z)
        })
        .collect()
}

fn default_input() -> FxHashSet<Vector3> {
    parse(include_input!(2022 / 18))
}

const DELTAS: [Vector3; 6] = [
    Vector3::new(1, 0, 0),
    Vector3::new(-1, 0, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 0, 1),
    Vector3::new(0, 0, -1),
];

const DELTAS_FULL: [Vector3; 26] = [
    Vector3::new(-1, -1, -1),
    Vector3::new(0, -1, -1),
    Vector3::new(1, -1, -1),
    Vector3::new(-1, 0, -1),
    Vector3::new(0, 0, -1),
    Vector3::new(1, 0, -1),
    Vector3::new(-1, 1, -1),
    Vector3::new(0, 1, -1),
    Vector3::new(1, 1, -1),
    Vector3::new(-1, -1, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(1, -1, 0),
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(-1, 1, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(1, 1, 0),
    Vector3::new(-1, -1, 1),
    Vector3::new(0, -1, 1),
    Vector3::new(1, -1, 1),
    Vector3::new(-1, 0, 1),
    Vector3::new(0, 0, 1),
    Vector3::new(1, 0, 1),
    Vector3::new(-1, 1, 1),
    Vector3::new(0, 1, 1),
    Vector3::new(1, 1, 1),
];

fn part1(points: FxHashSet<Vector3>) -> i64 {
    points
        .iter()
        .flat_map(|p| DELTAS.map(|d| p + d))
        .filter(|p| !points.contains(p))
        .count() as i64
}

fn part2(points: FxHashSet<Vector3>) -> i64 {
    let top_point = *points.iter().max_by_key(|p| p.x).unwrap();
    let top_point = top_point + Vector3::E1;

    let mut steam = FxHashSet::default();
    steam.insert(top_point);
    let mut steam_frontier = FxHashSet::default();
    steam_frontier.insert(top_point);

    loop {
        let mut new_frontier = FxHashSet::default();

        for s in steam_frontier.iter() {
            for n in DELTAS.map(|d| d + s) {
                if !steam.contains(&n)
                    && !points.contains(&n)
                    && DELTAS_FULL
                        .iter()
                        .map(|d| d + n)
                        .any(|p| points.contains(&p))
                {
                    new_frontier.insert(n);
                }
            }
        }

        if new_frontier.is_empty() {
            break;
        }

        steam.extend(new_frontier.iter());
        steam_frontier.clear();
        steam_frontier.extend(new_frontier);
    }

    points
        .iter()
        .flat_map(|p| DELTAS.map(|d| p + d))
        .filter(|p| steam.contains(p))
        .count() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = parse(
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    assert_eq!(part1(input.clone()), 64);
    assert_eq!(part2(input), 58);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4390);
    assert_eq!(part2(input), 2534);
}
