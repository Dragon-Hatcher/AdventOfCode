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

fn part1(input: &str) -> String {
    fn bfs(key: &str) -> String {
        let start = Vector2::ZERO;
        let goal = Vector2::new(3, 3);

        let mut edge = HashSet::new();
        edge.insert((start, "".to_owned()));

        loop {
            let mut new_edge = HashSet::new();

            for (pos, path) in edge {
                if pos == goal {
                    return path;
                }

                let (u, d, l, r) = open_doors(key, &path);

                if u && pos.y != 0 {
                    new_edge.insert((pos - Vector2::E2, format!("{path}U")));
                }
                if d && pos.y != 3 {
                    new_edge.insert((pos + Vector2::E2, format!("{path}D")));
                }
                if l && pos.x != 0 {
                    new_edge.insert((pos - Vector2::E1, format!("{path}L")));
                }
                if r && pos.x != 3 {
                    new_edge.insert((pos + Vector2::E1, format!("{path}R")));
                }
            }

            edge = new_edge;
        }
    }

    bfs(input.trim())
}

fn part2(input: &str) -> i64 {
    fn bfs(key: &str) -> i64 {
        let start = Vector2::ZERO;
        let goal = Vector2::new(3, 3);

        let mut steps = 0;
        let mut longest = 0;
        let mut edge = HashSet::new();
        edge.insert((start, "".to_owned()));

        loop {
            if edge.is_empty() {
                return longest;
            }

            let mut new_edge = HashSet::new();

            for (pos, path) in edge {
                let (u, d, l, r) = open_doors(key, &path);

                if u && pos.y != 0 {
                    new_edge.insert((pos - Vector2::E2, format!("{path}U")));
                }
                if d && pos.y != 3 {
                    new_edge.insert((pos + Vector2::E2, format!("{path}D")));
                }
                if l && pos.x != 0 {
                    new_edge.insert((pos - Vector2::E1, format!("{path}L")));
                }
                if r && pos.x != 3 {
                    new_edge.insert((pos + Vector2::E1, format!("{path}R")));
                }
            }

            steps += 1;
            edge = new_edge;

            if edge.iter().any(|(p, _)| *p == goal) {
                longest = steps;
            }
            edge.retain(|(p, _)| *p != goal);
        }
    }

    bfs(input.trim())
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
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
