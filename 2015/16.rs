use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 16)
}

#[derive(Debug, Clone, Copy)]
struct Sue {
    children: Option<i64>,
    cats: Option<i64>,
    samoyeds: Option<i64>,
    pomeranians: Option<i64>,
    akitas: Option<i64>,
    vizslas: Option<i64>,
    goldfish: Option<i64>,
    trees: Option<i64>,
    cars: Option<i64>,
    perfumes: Option<i64>,
}

impl Sue {
    fn is_correct_part_1(self) -> bool {
        self.children.map(|v| v == 3).unwrap_or(true)
            && self.cats.map(|v| v == 7).unwrap_or(true)
            && self.samoyeds.map(|v| v == 2).unwrap_or(true)
            && self.pomeranians.map(|v| v == 3).unwrap_or(true)
            && self.akitas.map(|v| v == 0).unwrap_or(true)
            && self.vizslas.map(|v| v == 0).unwrap_or(true)
            && self.goldfish.map(|v| v == 5).unwrap_or(true)
            && self.trees.map(|v| v == 3).unwrap_or(true)
            && self.cars.map(|v| v == 2).unwrap_or(true)
            && self.perfumes.map(|v| v == 1).unwrap_or(true)
    }

    fn is_correct_part_2(self) -> bool {
        self.children.map(|v| v == 3).unwrap_or(true)
            && self.cats.map(|v| v > 7).unwrap_or(true)
            && self.samoyeds.map(|v| v == 2).unwrap_or(true)
            && self.pomeranians.map(|v| v < 3).unwrap_or(true)
            && self.akitas.map(|v| v == 0).unwrap_or(true)
            && self.vizslas.map(|v| v == 0).unwrap_or(true)
            && self.goldfish.map(|v| v < 5).unwrap_or(true)
            && self.trees.map(|v| v > 3).unwrap_or(true)
            && self.cars.map(|v| v == 2).unwrap_or(true)
            && self.perfumes.map(|v| v == 1).unwrap_or(true)
    }
}

fn parse_sue(str: &str) -> Sue {
    let (_, rest) = str.split_once(": ").unwrap();
    let mut sue = Sue {
        children: None,
        cats: None,
        samoyeds: None,
        pomeranians: None,
        akitas: None,
        vizslas: None,
        goldfish: None,
        trees: None,
        cars: None,
        perfumes: None,
    };
    for item in rest.split(", ") {
        let (item, cnt) = item.split_once(": ").unwrap();
        let cnt = Some(cnt.parse().unwrap());

        match item {
            "children" => sue.children = cnt,
            "cats" => sue.cats = cnt,
            "samoyeds" => sue.samoyeds = cnt,
            "pomeranians" => sue.pomeranians = cnt,
            "akitas" => sue.akitas = cnt,
            "vizslas" => sue.vizslas = cnt,
            "goldfish" => sue.goldfish = cnt,
            "trees" => sue.trees = cnt,
            "cars" => sue.cars = cnt,
            "perfumes" => sue.perfumes = cnt,
            _ => {}
        }
    }
    sue
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_sue)
        .position(Sue::is_correct_part_1)
        .unwrap() as i64
        + 1
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_sue)
        .position(Sue::is_correct_part_2)
        .unwrap() as i64
        + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 373);
    assert_eq!(part2(input), 260);
}
