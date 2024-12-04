use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 04)
}

fn part1(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    let a = |p: Vec2| vec![p, p + v2(1, 0), p + v2(2, 0), p + v2(3, 0)];
    let b = |p: Vec2| vec![p, p + v2(0, 1), p + v2(0, 2), p + v2(0, 3)];
    let e = |p: Vec2| vec![p, p + v2(1, 1), p + v2(2, 2), p + v2(3, 3)];
    let f = |p: Vec2| vec![p, p + v2(1, -1), p + v2(2, -2), p + v2(3, -3)];
    let c = |p: Vec2| vec![p, p - v2(1, 0), p - v2(2, 0), p - v2(3, 0)];
    let d = |p: Vec2| vec![p, p - v2(0, 1), p - v2(0, 2), p - v2(0, 3)];
    let g = |p: Vec2| vec![p, p - v2(1, 1), p - v2(2, 2), p - v2(3, 3)];
    let h = |p: Vec2| vec![p, p - v2(1, -1), p - v2(2, -2), p - v2(3, -3)];

    let mut s = 0;
    for p in grid.points() {
        if a(p).iter().all(|l| grid.in_bounds(*l))
            && grid[a(p)[0]] == 'X'
            && grid[a(p)[1]] == 'M'
            && grid[a(p)[2]] == 'A'
            && grid[a(p)[3]] == 'S'
        {
            s += 1
        }

        if b(p).iter().all(|l| grid.in_bounds(*l))
            && grid[b(p)[0]] == 'X'
            && grid[b(p)[1]] == 'M'
            && grid[b(p)[2]] == 'A'
            && grid[b(p)[3]] == 'S'
        {
            s += 1
        }

        if c(p).iter().all(|l| grid.in_bounds(*l))
            && grid[c(p)[0]] == 'X'
            && grid[c(p)[1]] == 'M'
            && grid[c(p)[2]] == 'A'
            && grid[c(p)[3]] == 'S'
        {
            s += 1
        }

        if d(p).iter().all(|l| grid.in_bounds(*l))
            && grid[d(p)[0]] == 'X'
            && grid[d(p)[1]] == 'M'
            && grid[d(p)[2]] == 'A'
            && grid[d(p)[3]] == 'S'
        {
            s += 1
        }

        if e(p).iter().all(|l| grid.in_bounds(*l))
            && grid[e(p)[0]] == 'X'
            && grid[e(p)[1]] == 'M'
            && grid[e(p)[2]] == 'A'
            && grid[e(p)[3]] == 'S'
        {
            s += 1
        }

        if f(p).iter().all(|l| grid.in_bounds(*l))
            && grid[f(p)[0]] == 'X'
            && grid[f(p)[1]] == 'M'
            && grid[f(p)[2]] == 'A'
            && grid[f(p)[3]] == 'S'
        {
            s += 1
        }

        if g(p).iter().all(|l| grid.in_bounds(*l))
            && grid[g(p)[0]] == 'X'
            && grid[g(p)[1]] == 'M'
            && grid[g(p)[2]] == 'A'
            && grid[g(p)[3]] == 'S'
        {
            s += 1
        }

        if h(p).iter().all(|l| grid.in_bounds(*l))
        && grid[h(p)[0]] == 'X'
        && grid[h(p)[1]] == 'M'
        && grid[h(p)[2]] == 'A'
        && grid[h(p)[3]] == 'S'
    {
        s += 1
    }
    }

    s
}

fn part2(input: &str) -> i64 {
    let grid = Grid::new_by_char(input, |c| c);

    let mut s = 0;
    for p in grid.points() {
        if !grid.in_bounds(p - v2(1, 1)) { continue;}
        if !grid.in_bounds(p - v2(1, -1)) { continue;}
        if !grid.in_bounds(p - v2(-1, 1)) { continue;}
        if !grid.in_bounds(p - v2(-1, -1)) { continue;}

        if grid[p] != 'A' { continue;}

        if !(grid[p - v2(1, 1)] == 'M' && grid[p + v2(1, 1)] == 'S' ||
        grid[p - v2(1, 1)] == 'S' && grid[p + v2(1, 1)] == 'M') {
            continue;
        }

        if !(grid[p - v2(1, -1)] == 'M' && grid[p + v2(1, -1)] == 'S' ||
        grid[p - v2(1, -1)] == 'S' && grid[p + v2(1, -1)] == 'M') {
            continue;
        }

        s += 1;
    }

    s
}

fn main() {
    advent::new(2024, 04, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(input), 18);
    assert_eq!(part2(input), 9);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2427);
    assert_eq!(part2(input), 1900);
}
