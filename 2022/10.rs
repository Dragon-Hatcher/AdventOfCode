use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 10)
}

fn part1(input: &str) -> i64 {
    let mut cycle_num = 0;
    let mut x = 1;
    let mut tot = 0;

    let mut cycle = |x: i64| {
        cycle_num += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycle_num) {
            tot += x * cycle_num;
        }
    };

    for l in input.lines() {
        if l == "noop" {
            cycle(x);
        } else {
            cycle(x);
            cycle(x);
            x += l.nums().nu();
        }
    }

    tot
}

fn part2(input: &str) -> String {
    let mut cycle_num = 0;
    let mut x: i64 = 1;
    let mut out = "\n".to_owned();

    let mut cycle = |x: i64| {
        let char = if (x - cycle_num % 40).abs() < 2 {
            '#'
        } else {
            '.'
        };
        out.push(char);
        cycle_num += 1;
        if cycle_num % 40 == 0 {
            out.push('\n');
        }
    };

    for l in input.lines() {
        if l == "noop" {
            cycle(x);
        } else {
            cycle(x);
            cycle(x);
            x += l.nums().nu();
        }
    }

    out
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part1(input), 13140);
    assert_eq!(
        part2(input),
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 17020);
    assert_eq!(
        part2(input),
        "
###..#....####.####.####.#.....##..####.
#..#.#....#.......#.#....#....#..#.#....
#..#.#....###....#..###..#....#....###..
###..#....#.....#...#....#....#.##.#....
#.#..#....#....#....#....#....#..#.#....
#..#.####.####.####.#....####..###.####.
"
    );
}
