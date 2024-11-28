use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 22)
}

#[derive(Debug, Clone, Copy)]
struct Node {
    used: i64,
    avail: i64,
}

impl Node {
    fn size(&self) -> i64 {
        self.used + self.avail
    }
}

fn parse_nodes(input: &str) -> HashMap<Vec2, Node> {
    input
        .lines()
        .skip(2)
        .map(|l| {
            let (x, y, _size, used, avail) = l.nums_pos().tup();
            (Vec2::new(x, y), Node { used, avail })
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let nodes = parse_nodes(input);

    nodes
        .iter()
        .permutations(2)
        .map(|p| p.into_iter().tup())
        .filter(|((p1, n1), (p2, n2))| p1 != p2 && n1.used != 0 && n1.used < n2.avail)
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let nodes = parse_nodes(input);
    let max_x = nodes.keys().map(|p| p.x).max().unwrap_or_default();

    const LARGE_SIZE: i64 = 100;
    let wall_left = nodes
        .iter()
        .filter(|(_, n)| n.size() > LARGE_SIZE)
        .map(|(p, _)| p.x)
        .min()
        .unwrap_or_default();

    let (zero_loc, _) = nodes.iter().find(|(_, n)| n.used == 0).unwrap();

    let dist = (zero_loc.x - wall_left + 1) + zero_loc.y + (max_x - wall_left + 1);
    let moves = max_x - 1;
    dist + moves * 5
}
fn main() {
    advent::new(2016, 22, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 934);
    assert_eq!(part2(input), 207);
}
