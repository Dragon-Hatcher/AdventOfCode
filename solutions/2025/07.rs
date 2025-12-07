use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2025 / 07)
}

fn parse(input: &str) -> (Grid<bool>, Vec2) {
    let splitters = Grid::new_by_char(input, |c| c == '^');
    let start = Grid::new_by_char(input, |s| s == 'S');
    let start = start.points().find(|&p| start[p]).unwrap();
    (splitters, start)
}

fn part1(input: &str) -> i64 {
    let (splitters, start) = parse(input);

    let mut seen = HashSet::default();
    let mut stack = vec![start];
    let mut split_count = 0;

    while let Some(next) = stack.pop() {
        let down = next + Vec2::new(0, 1);
        let Some(splitter) = splitters.get(down) else {
            continue;
        };

        let mut add = |v: Vec2| {
            if seen.insert(v) {
                stack.push(v);
            }
        };

        if *splitter {
            add(down - Vec2::new(1, 0));
            add(down + Vec2::new(1, 0));
            split_count += 1;
        } else {
            add(down);
        }
    }

    split_count
}

fn part2(input: &str) -> i64 {
    let (splitters, start) = parse(input);

    fn solve(p: Vec2, splitters: &Grid<bool>, memo: &mut HashMap<Vec2, i64>) -> i64 {
        if let Some(memo) = memo.get(&p) {
            return *memo;
        }

        let down = p + Vec2::new(0, 1);
        let answer = match splitters.get(down) {
            Some(true) => {
                solve(down - Vec2::new(1, 0), splitters, memo)
                    + solve(down + Vec2::new(1, 0), splitters, memo)
            }
            Some(false) => solve(down, splitters, memo),
            None => 1,
        };

        memo.insert(p, answer);
        answer
    }

    solve(start, &splitters, &mut HashMap::default())
}

fn main() {
    advent::new(2025, 7, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 40);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
