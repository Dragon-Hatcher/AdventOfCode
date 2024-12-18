use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!("width=71\n\n" / 2024 / 18)
}

fn part1(input: &str) -> i64 {
    let (width, bytes) = input.sections().tup();
    let width = width.nums().nu();
    let bytes = bytes.lines().map(|l| {
        let (x, y) = l.nums().tup();
        v2(x, y)
    }).collect_vec();

    let mut grid = Grid::new_homogenous(width, width, false);

    for p in bytes.into_iter().take(1024) {
        grid[p] = true;
    }

    println!("{}", grid.pretty());
    dbg!(v2(width, width));

    bfs()
        .start(v2(0, 0))
        .goal(v2(width - 1, width - 1))
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .shortest()
        .steps
}

fn part2(input: &str) -> i64 {
    let (width, bytes) = input.sections().tup();
    let width = width.nums().nu();
    let bytes = bytes.lines().map(|l| {
        let (x, y) = l.nums().tup();
        v2(x, y)
    }).collect_vec();

    let mut grid = Grid::new_homogenous(width, width, false);

    for p in bytes.into_iter() {
        grid[p] = true;

        if bfs()
        .start(v2(0, 0))
        .goal(v2(width - 1, width - 1))
        .next(|p| grid.neighbors4(*p).filter(|p| !grid[*p]))
        .try_shortest().is_err() {
            dbg!(p);
            return 1;
        }
    }

    unreachable!()
}

fn main() {
    advent::new(2024, 18, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "width=7

5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    // assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
