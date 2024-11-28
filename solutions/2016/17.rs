use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 17)
}

fn open_doors(key: &str, path: &str) -> (bool, bool, bool, bool) {
    let test = format!("{key}{path}");
    let md5 = format!("{:032x}", md5_str(test));
    let mut chars = md5.chars();

    (
        chars.nu() >= 'b',
        chars.nu() >= 'b',
        chars.nu() >= 'b',
        chars.nu() >= 'b',
    )
}

fn next(key: &str, pos: Vec2, path: &str) -> Vec<(Vec2, String)> {
    let (u, d, l, r) = open_doors(key, path);
    let mut next = Vec::with_capacity(4);

    if u && pos.y != 0 {
        next.push((pos - Vec2::E2, format!("{path}U")));
    }
    if d && pos.y != 3 {
        next.push((pos + Vec2::E2, format!("{path}D")));
    }
    if l && pos.x != 0 {
        next.push((pos - Vec2::E1, format!("{path}L")));
    }
    if r && pos.x != 3 {
        next.push((pos + Vec2::E1, format!("{path}R")));
    }

    next
}

fn part1(input: &str) -> String {
    let key = input.trim();

    let bfs = bfs()
        .start((Vec2::ZERO, "".to_owned()))
        .is_goal(|n| n.0 == Vec2::new(3, 3))
        .next(|(pos, path)| next(key, *pos, path));

    bfs.solve().node.1
}

fn part2(input: &str) -> i64 {
    let key = input.trim();
    let goal = Vec2::new(3, 3);

    let bfs = bfs()
        .start((Vec2::ZERO, "".to_owned()))
        .no_goal()
        .next(|(pos, path)| next(key, *pos, path));

    bfs.finish()
        .visited
        .into_iter()
        .filter(|((pos, _), _)| pos == &goal)
        .map(|(_, dist)| dist)
        .max()
        .unwrap_or_default()
}

fn main() {
    advent::new(2016, 17, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "ihgpwlah";
    assert_eq!(part1(input), "DDRRRD");
    assert_eq!(part2(input), 370);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "DRDRULRDRD");
    assert_eq!(part2(input), 384);
}
