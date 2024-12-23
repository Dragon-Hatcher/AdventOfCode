use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("dist=10000\n\n" / 2018 / 06)
}

fn part1(input: &str) -> i64 {
    let (_, points) = input.sections().tup();
    let points = points
        .lines()
        .map(|l| {
            let (x, y) = l.nums().tup();
            Vec2::new(x, y)
        })
        .collect_vec();

    let (x_min, x_max) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
    let (y_min, y_max) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
    let least = Vec2::new(x_min, y_min);
    let range = Vec2::new(x_max - x_min + 1, y_max - y_min + 1);
    let points = points.into_iter().map(|p| p - least).collect_vec();

    let range = Range::new_size(range.x, range.y);
    let mut sizes: HashMap<usize, i64> = HashMap::default();
    let mut infinite = HashSet::default();

    for p in range.points() {
        let dists = points.iter().map(|pp| pp.manhattan_dist(p));
        let first = dists.clone().position_min().unwrap();
        let last = points.len() - 1 - dists.rev().position_min().unwrap();

        if first != last {
            continue;
        }

        *sizes.entry(first).or_default() += 1;

        if p.x == 0 || p.y == 0 || p.x == range.width() - 1 || p.y == range.height() - 1 {
            infinite.insert(first);
        }
    }

    dbg!(&infinite);

    sizes
        .into_iter()
        .filter(|(target, _size)| !infinite.contains(target))
        .map(|(_target, size)| size)
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> i64 {
    let (dist, points) = input.sections().tup();
    let dist = dist.nums().nu();
    let points = points
        .lines()
        .map(|l| {
            let (x, y) = l.nums().tup();
            Vec2::new(x, y)
        })
        .collect_vec();

    let x_min = points.iter().map(|p| p.x).min().unwrap();
    let y_min = points.iter().map(|p| p.y).min().unwrap();
    let least = Vec2::new(x_min, y_min);
    let range = Vec2::new(dist / points.len() as i64, dist / points.len() as i64);
    let points = points.into_iter().map(|p| p - least + range).collect_vec();

    let range = Range::new_size(range.x * 3, range.y * 3);

    let mut sum = 0;
    for p in range.points() {
        let total_dist: i64 = points.iter().map(|target| target.manhattan_dist(p)).sum();
        if total_dist < dist {
            sum += 1;
        }
    }

    sum
}

fn main() {
    advent::new(2018, 6, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "dist=32

1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(part1(input), 17);
    assert_eq!(part2(input), 16);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 4186);
    assert_eq!(part2(input), 45509);
}
