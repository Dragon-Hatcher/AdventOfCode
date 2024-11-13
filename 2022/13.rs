use std::iter::once;

use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 13)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Num(i64),
    List(Vec<Element>),
}

impl Element {
    fn wrap(self) -> Element {
        Element::List(vec![self])
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        use Element::{List, Num};

        match (self, other) {
            (Num(l), Num(r)) => l.cmp(r),
            (Num(l), List(_)) => Num(*l).wrap().cmp(other),
            (List(_), Num(r)) => self.cmp(&Num(*r).wrap()),
            (List(l), List(r)) => {
                for i in 0..l.len() {
                    if i >= r.len() {
                        return Ordering::Greater;
                    }

                    let o = l[i].cmp(&r[i]);
                    match o {
                        Ordering::Less | Ordering::Greater => return o,
                        Ordering::Equal => continue,
                    }
                }

                l.len().cmp(&r.len())
            }
        }
    }
}

fn parse(input: &str) -> Element {
    fn parse(input: &mut impl Iterator<Item = char>) -> Element {
        let mut elements = Vec::new();
        let mut dig = "".to_owned();

        while let Some(c) = input.next() {
            match c {
                '[' => elements.push(parse(input)),
                ',' => {
                    if let Ok(num) = dig.parse::<i64>() {
                        elements.push(Element::Num(num));
                    }
                    dig = "".to_owned();
                }
                ']' => {
                    if let Ok(num) = dig.parse::<i64>() {
                        elements.push(Element::Num(num));
                    }
                    return Element::List(elements);
                }
                _ => dig.push(c),
            }
        }

        elements.pop().unwrap()
    }

    parse(&mut input.chars())
}

fn part1(input: &str) -> i64 {
    input
        .sections()
        .enumerate()
        .filter(|(_, section)| {
            let (left, right) = section.lines().map(parse).tup();
            left < right
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>() as i64
}

fn part2(input: &str) -> i64 {
    let key1 = Element::Num(2).wrap().wrap();
    let key2 = Element::Num(6).wrap().wrap();

    input
        .non_empty()
        .map(parse)
        .chain(once(key1.clone()))
        .chain(once(key2.clone()))
        .sorted()
        .enumerate()
        .filter(|(_, e)| *e == key1 || *e == key2)
        .map(|(i, _)| i + 1)
        .product::<usize>() as i64
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 140);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 5605);
    assert_eq!(part2(input), 24969);
}
