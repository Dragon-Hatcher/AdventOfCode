use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 14)
}

struct ReindeerStats {
    speed: i64,
    endurance: i64,
    rest_time: i64,
}

struct Reindeer {
    stats: ReindeerStats,
    distance: i64,
    points: i64,
    flying: bool,
    time_left: i64,
}

impl Reindeer {
    fn iterate(&mut self) {
        if self.flying {
            self.distance += self.stats.speed;
        }
        self.time_left -= 1;

        if self.time_left == 0 {
            self.flying = !self.flying;
            self.time_left = if self.flying {
                self.stats.endurance
            } else {
                self.stats.rest_time
            };
        }
    }
}

fn parse_reinder(str: &str) -> Reindeer {
    let (speed, endurance, rest_time) = str.nums().tup();
    Reindeer {
        stats: ReindeerStats {
            speed,
            endurance,
            rest_time,
        },
        distance: 0,
        points: 0,
        flying: true,
        time_left: endurance,
    }
}

fn max_dist(reindeer: &[Reindeer]) -> i64 {
    reindeer
        .iter()
        .map(|r| r.distance)
        .max()
        .unwrap_or_default()
}

fn part1(input: &str) -> i64 {
    let mut reindeer = input.lines().map(parse_reinder).collect_vec();

    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.iterate();
        }
    }

    max_dist(&reindeer)
}

fn part2(input: &str) -> i64 {
    let mut reindeer = input.lines().map(parse_reinder).collect_vec();

    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.iterate();
        }
        let max = max_dist(&reindeer);
        for r in reindeer.iter_mut() {
            if r.distance == max {
                r.points += 1;
            }
        }
    }

    reindeer
        .iter()
        .map(|r| r.points)
        .max()
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2660);
    assert_eq!(part2(input), 1256);
}
