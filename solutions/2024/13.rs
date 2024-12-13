use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 13)
}

fn part1(input: &str) -> i64 {
    let mut vec = vec![];

    for sec in input.sections() {
        let (ax, ay, bx, by, px, py) = sec.nums().tup();
        vec.push((v2(ax, ay), v2(bx, by), v2(px, py)));
    }

    let mut sum = 0;

    'outer: for (a, b, p) in vec {
        // dbg!(a, b, p);
        for t in 1..=200 {
            for ap in 0..=t {
                let bp = t - ap;
                if p == a * ap + b * bp {
                    sum += 3 * ap + bp;
                    continue 'outer;
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut vec = vec![];

    for sec in input.sections() {
        let (ax, ay, bx, by, px, py) = sec.nums().tup();
        vec.push((
            v2(ax, ay),
            v2(bx, by),
            v2(px + 10000000000000, py + 10000000000000),
        ));
    }

    let mut sum = 0;

    for (a, b, p) in vec {
        let delta_x = a.x - b.x;
        let delta_y = a.y - b.y;

        let num = p.y * delta_x - p.x * delta_y;
        let denom = delta_x * b.y - delta_y * b.x;

        if num % denom != 0 {
            continue;
        }

        let t = num / denom;

        let diff = p.x - t * b.x;
        let div = a.x - b.x;

        let ap = diff / div;
        let bp = t - ap;

        sum += 3 * ap + bp;
    }

    sum
}

fn main() {
    advent::new(2024, 13, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part1(input), 480);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 32026);
    assert_eq!(part2(input), 89013607072065);
}
