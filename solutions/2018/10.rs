use std::i64;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2018 / 10)
}

#[derive(Debug, Clone, Copy)]
struct Light {
    pos: Vec2,
    vel: Vec2
}

impl Light {
    fn tick(&mut self) {
        self.pos += self.vel;
    }

    fn un_tick(&mut self) {
        self.pos -= self.vel;
    }
}

fn parse_lights(input: &str) -> Vec<Light> {
    input
        .lines()
        .map(|l| {
            let (x, y, dx, dy) = l.nums().tup();
            Light { pos: Vec2::new(x, y), vel: Vec2::new(dx, dy) }
        })
        .collect()
}

fn bounding_box(lights: &[Light]) -> Range {
    let (x_min, x_max) = lights.iter().map(|l| l.pos.x).minmax().into_option().unwrap();
    let (y_min, y_max) = lights.iter().map(|l| l.pos.y).minmax().into_option().unwrap();
    Range::new_tl_br(Vec2::new(x_min, y_min), Vec2::new(x_max, y_max))
}

fn debug(lights: &[Light], bounds: Range) {
    let mut grid = Grid::new_homogenous(bounds.width(), bounds.height(), false);

    for light in lights {
        grid[light.pos - bounds.top_left()] = true;
    }

    println!("{}", grid.pretty());
}

fn part1(input: &str) -> i64 {
    let mut lights = parse_lights(input);
    let mut last_area = i64::MAX;

    loop {
        lights.iter_mut().for_each(Light::tick);
        let bounds = bounding_box(&lights);

        if bounds.area() > last_area {
            lights.iter_mut().for_each(Light::un_tick);
            debug(&lights, bounding_box(&lights));
            break;
        }

        last_area = last_area.min(bounds.area());
    }

    1
}

fn part2(input: &str) -> i64 {
    let mut lights = parse_lights(input);
    let mut last_area = i64::MAX;
    let mut steps = 0;

    loop {
        lights.iter_mut().for_each(Light::tick);
        let bounds = bounding_box(&lights);

        if bounds.area() > last_area {
            lights.iter_mut().for_each(Light::un_tick);
            debug(&lights, bounding_box(&lights));
            break;
        }

        steps += 1;
        last_area = last_area.min(bounds.area());
    }

    steps
}

fn main() {
    advent::new(2018, 10, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    assert_eq!(part1(input), 1);
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 10645);
}
