use std::{i64, thread, time::Duration};

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 14)
}

fn part1(input: &str) -> i64 {
    let mut bots = input
        .lines()
        .map(|l| {
            let (px, py, vx, vy) = l.nums().tup();
            (v2(px, py), v2(vx, vy))
        })  
        .collect_vec();

    for _ in 0..100 {
        for (p, v) in bots.iter_mut() {
            *p += *v;
            *p = v2(p.x.rem_euclid(101), p.y.rem_euclid(103));
        }
    }

    let mut counts = [0; 4];

    for (p, _) in bots {
        if p.x == 101 / 2 || p.y == 103 / 2 {
            continue;
        }

        // dbg!(p);
        let c = (p.x / 51) + 2 * (p.y / 52);
        counts[c as usize] += 1;
    }


    counts.into_iter().product()
}

fn bounding_box(lights: &[(Vec2, Vec2)]) -> Range {
    let (x_min, x_max) = lights.iter().map(|l| l.0.x).minmax().into_option().unwrap();
    let (y_min, y_max) = lights.iter().map(|l| l.0.y).minmax().into_option().unwrap();
    Range::new_tl_br(Vec2::new(x_min, y_min), Vec2::new(x_max, y_max))
}


fn part2(input: &str) -> i64 {
    let mut bots = input
        .lines()
        .map(|l| {
            let (px, py, vx, vy) = l.nums().tup();
            (v2(px, py), v2(vx, vy))
        })  
        .collect_vec();

        let mut min_area = 1000000000;
        let mut min_str = String::new();
    for i in 0.. {
        for (p, v) in bots.iter_mut() {
            *p += *v;
            *p = v2(p.x.rem_euclid(101), p.y.rem_euclid(103));
        }



        let mut grid = Grid::new_homogenous(101, 103, false);
        for (p, _) in bots.iter() {
            grid[*p] = true;
        }

        let mut horiz = 0;
        let mut vert = 0;

        for y in 0..grid.height() {
            if grid.row_range(y).points().filter(|p| grid[*p]).count() > 15 {
                horiz += 1;
            }
        }

        for x in 0..grid.width() {
            if grid.col_range(x).points().filter(|p| grid[*p]).count() > 15 {
                vert += 1;
            }
        }

        if horiz >= 2 && vert >= 2 {
            println!("{}:\n{}", i+1, grid.pretty());
            panic!();
        }

        // let x_avg = bots.iter().map(|t| t.0.x).sum::<i64>() / bots.len() as i64;
        // let y_avg = bots.iter().map(|t| t.0.y).sum::<i64>() / bots.len() as i64;

        // let x_dev = bots.iter().map(|t| (t.0.x - x_avg) * (t.0.x - x_avg)).sum::<i64>() / bots.len() as i64;
        // let y_dev = bots.iter().map(|t| (t.0.y - y_avg) * (t.0.y - y_avg)).sum::<i64>() / bots.len() as i64;

        // let diff = x_dev + y_dev;




        // // // println!("{}:\n{}", i+1, grid.pretty());


        // // let bounds = bounding_box(&bots);
        // // // if bounds.area() > min_area + 5000 {
        // // //     panic!();
        // // // }
        // if diff < 1500 {
        //     println!("{} {}:\n{}", i+1, diff, min_str);
        //     min_str = grid.pretty();
        //     min_area = min_area.min(diff);
        //     dbg!(min_area);

        //     thread::sleep(Duration::from_millis(1000));

        // }


    }

    // let mut counts = [0; 4];

    // for (p, _) in bots {
    //     if p.x == 101 / 2 || p.y == 103 / 2 {
    //         continue;
    //     }

    //     dbg!(p);
    //     let c = (p.x / 51) + 2 * (p.y / 52);
    //     counts[c as usize] += 1;
    // }


    0
}

fn main() {
    advent::new(2024, 14, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "";
    assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    // assert_eq!(part2(input), 0);
}
