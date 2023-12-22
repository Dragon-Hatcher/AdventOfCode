use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 22)
}

#[derive(Debug, Clone, Copy)]
struct Brick(Vector3, Vector3);

impl Brick {
    fn lower(mut self) -> Self {
        self.0.z -= 1;
        self.1.z -= 1;
        self
    }

    fn above_ground(self) -> bool {
        self.0.z > 0 && self.1.z > 0
    }

    fn points(self) -> impl Iterator<Item = Vector3> {
        iproduct!(
            self.0.x..=self.1.x,
            self.0.y..=self.1.y,
            self.0.z..=self.1.z
        )
        .map(|(x, y, z)| Vector3::new(x, y, z))
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) = l.nums().tup();
            Brick(Vector3::new(x1, y1, z1), Vector3::new(x2, y2, z2))
        })
        .collect()
}

fn create_map(bricks: &[Brick]) -> HashMap<Vector3, usize> {
    let mut map: HashMap<Vector3, usize> = HashMap::new();

    for (group, mut brick) in bricks.iter().copied().enumerate() {
        while {
            let low = brick.lower();
            low.above_ground() && low.points().all(|p| !map.contains_key(&p))
        } {
            brick = brick.lower();
        }

        for p in brick.points() {
            map.insert(p, group);
        }
    }

    map
}

fn get_supports(map: &HashMap<Vector3, usize>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (p, &group) in map {
        let down = Vector3::new(p.x, p.y, p.z - 1);
        if let Some(&dg) = map.get(&down) {
            if dg != group {
                supporting.entry(dg).or_default().insert(group);
                supported_by.entry(group).or_default().insert(dg);
            }
        }
    }

    (supporting, supported_by)
}

fn part1(input: &str) -> i64 {
    let mut bricks = parse(input);
    bricks.sort_by_key(|b| b.0.z);

    let map = create_map(&bricks);
    let (supporting, supported_by) = get_supports(&map);

    (0..bricks.len())
        .filter(|g| {
            supporting
                .get(g)
                .map(|supporting| {
                    supporting
                        .iter()
                        .all(|g| supported_by.get(g).unwrap().len() > 1)
                })
                .unwrap_or(true)
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let mut bricks = parse(input);
    bricks.sort_by_key(|b| b.0.z);

    let map = create_map(&bricks);
    let (supporting, supported_by) = get_supports(&map);

    let mut sum = 0;

    for group in 0..bricks.len() {
        let mut fallen = HashSet::new();
        fallen.insert(group);

        loop {
            let next: HashSet<usize> = fallen
                .iter()
                .copied()
                .flat_map(|g| supporting.get(&g).cloned().unwrap_or_default())
                .filter(|g| !fallen.contains(g))
                .filter(|g| supported_by.get(g).unwrap().difference(&fallen).count() == 0)
                .collect();

            if next.is_empty() {
                sum += fallen.len() - 1;
                break;
            }

            fallen.extend(next);
        }
    }

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
