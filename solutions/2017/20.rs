use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 20)
}

#[derive(Debug)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn tick(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    fn is_steady_state(&self) -> bool {
        let p = (self.pos.x > 0, self.pos.y > 0, self.pos.z > 0);
        let v = (self.vel.x > 0, self.vel.y > 0, self.vel.z > 0);
        let a = (self.acc.x > 0, self.acc.y > 0, self.acc.z > 0);

        let x = p.0 == v.0 && (v.0 == a.0 || self.acc.x == 0);
        let y = p.1 == v.1 && (v.1 == a.1 || self.acc.y == 0);
        let z = p.2 == v.2 && (v.2 == a.2 || self.acc.z == 0);

        x && y && z
    }
}

fn parse_particle(line: &str) -> Particle {
    let (px, py, pz, vx, vy, vz, ax, ay, az) = line.nums().tup();
    Particle {
        pos: v3(px, py, pz),
        vel: v3(vx, vy, vz),
        acc: v3(ax, ay, az),
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_particle)
        .enumerate()
        .min_by_key(|(_, p)| p.acc.manhattan_mag())
        .unwrap()
        .0 as i64
}

fn part2(input: &str) -> i64 {
    let mut particles = input.lines().map(parse_particle).collect_vec();

    while !particles.iter().all(Particle::is_steady_state) {
        particles.iter_mut().for_each(Particle::tick);

        let mut seen_once = HashSet::default();
        let mut seen_twice = HashSet::default();

        for p in &particles {
            if !seen_once.insert(p.pos) {
                seen_twice.insert(p.pos);
            }
        }

        particles.retain(|p| !seen_twice.contains(&p.pos));
    }

    particles.len() as i64
}

fn main() {
    advent::new(2017, 20, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input1 = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
                        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
    assert_eq!(part1(input1), 0);

    let input2 = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
                        p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
                        p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
                        p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
    assert_eq!(part2(input2), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 144);
    assert_eq!(part2(input), 477);
}
