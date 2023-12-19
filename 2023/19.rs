use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 19)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn parse_item(item: &str) -> Item {
    let (x, m, a, s) = item.nums().tup();
    Item { x, m, a, s }
}

impl Item {
    fn get(&self, aspect: &Aspect) -> i64 {
        match aspect {
            Aspect::X => self.x,
            Aspect::M => self.m,
            Aspect::A => self.a,
            Aspect::S => self.s,
        }
    }

    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeItem {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl RangeItem {
    fn is_empty(&self) -> bool {
        self.x.0 > self.x.1 || self.m.0 > self.m.1 || self.a.0 > self.a.1 || self.s.0 > self.s.1
    }

    fn size(&self) -> i64 {
        (self.x.1 - self.x.0 + 1).max(0)
            * (self.m.1 - self.m.0 + 1).max(0)
            * (self.a.1 - self.a.0 + 1).max(0)
            * (self.s.1 - self.s.0 + 1).max(0)
    }

    fn split(self, aspect: &Aspect, upper: i64) -> (RangeItem, RangeItem) {
        match aspect {
            Aspect::X => (
                RangeItem {
                    x: (self.x.0, upper),
                    ..self
                },
                RangeItem {
                    x: (upper + 1, self.x.1),
                    ..self
                },
            ),
            Aspect::M => (
                RangeItem {
                    m: (self.m.0, upper),
                    ..self
                },
                RangeItem {
                    m: (upper + 1, self.m.1),
                    ..self
                },
            ),
            Aspect::A => (
                RangeItem {
                    a: (self.a.0, upper),
                    ..self
                },
                RangeItem {
                    a: (upper + 1, self.a.1),
                    ..self
                },
            ),
            Aspect::S => (
                RangeItem {
                    s: (self.s.0, upper),
                    ..self
                },
                RangeItem {
                    s: (upper + 1, self.s.1),
                    ..self
                },
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Aspect {
    X,
    M,
    A,
    S,
}

fn parse_aspect(a: &str) -> Aspect {
    match a {
        "x" => Aspect::X,
        "m" => Aspect::M,
        "a" => Aspect::A,
        _ => Aspect::S,
    }
}

#[derive(Debug, Clone)]
enum Action<'a> {
    Accept,
    Reject,
    Send(&'a str),
}

fn parse_action(a: &str) -> Action<'_> {
    match a {
        "A" => Action::Accept,
        "R" => Action::Reject,
        _ => Action::Send(a),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Unconditional,
    LT { aspect: Aspect, val: i64 },
    GT { aspect: Aspect, val: i64 },
}

impl Condition {
    fn matches(&self, item: &Item) -> bool {
        match self {
            Condition::Unconditional => true,
            Condition::LT { aspect, val } => item.get(aspect) < *val,
            Condition::GT { aspect, val } => item.get(aspect) > *val,
        }
    }
}

fn parse_cond(c: &str) -> Condition {
    if let Some((aspect, val)) = c.split_once('<') {
        let aspect = parse_aspect(aspect);
        let val: i64 = val.parse().unwrap();
        Condition::LT { aspect, val }
    } else if let Some((aspect, val)) = c.split_once('>') {
        let aspect = parse_aspect(aspect);
        let val: i64 = val.parse().unwrap();
        Condition::GT { aspect, val }
    } else {
        panic!()
    }
}

fn parse_cond_action(c: &str) -> (Condition, Action<'_>) {
    if let Some((cond, action)) = c.split_once(':') {
        let cond = parse_cond(cond);
        let action = parse_action(action);
        (cond, action)
    } else {
        (Condition::Unconditional, parse_action(c))
    }
}

fn parse_workflow(w: &str) -> (&str, Vec<(Condition, Action<'_>)>) {
    let (name, conditions) = w.split_once('{').unwrap();
    let conditions = conditions.strip_suffix('}').unwrap();
    let conditions = conditions.split(',').map(parse_cond_action).collect();
    (name, conditions)
}

fn part1(input: &str) -> i64 {
    let (workflows, items) = input.sections().tup();
    let workflows: HashMap<_, _> = workflows.lines().map(parse_workflow).collect();
    let items: Vec<_> = items.lines().map(parse_item).collect();

    items
        .iter()
        .filter(|i| {
            let mut cur = "in";

            'main: loop {
                let flow = workflows.get(cur).unwrap();
                for (cond, act) in flow {
                    if cond.matches(i) {
                        match act {
                            Action::Accept => break 'main true,
                            Action::Reject => break 'main false,
                            Action::Send(flow) => {
                                cur = flow;
                                break;
                            }
                        }
                    }
                }
            }
        })
        .map(|i| i.rating())
        .sum()
}

fn part2(input: &str) -> i64 {
    let (workflows, _) = input.sections().tup();
    let workflows: HashMap<_, _> = workflows.lines().map(parse_workflow).collect();

    let mut tot_accepted = 0;
    let mut ranges = Vec::new();
    ranges.push((
        "in",
        0,
        RangeItem {
            x: (1, 4000),
            a: (1, 4000),
            m: (1, 4000),
            s: (1, 4000),
        },
    ));

    while let Some((w_name, pos, r)) = ranges.pop() {
        if r.is_empty() {
            continue;
        }

        let flow = workflows.get(w_name).unwrap();
        let (cond, act) = &flow[pos];

        let (accepted, rejected) = match cond {
            Condition::Unconditional => (r, None),
            Condition::LT { aspect, val } => {
                let (accept, reject) = r.split(aspect, val - 1);
                (accept, Some(reject))
            }
            Condition::GT { aspect, val } => {
                let (reject, accept) = r.split(aspect, *val);
                (accept, Some(reject))
            }
        };

        match act {
            Action::Accept => tot_accepted += accepted.size(),
            Action::Reject => {}
            Action::Send(to) => ranges.push((to, 0, accepted)),
        }

        if let Some(rejected) = rejected {
            ranges.push((w_name, pos + 1, rejected))
        }
    }

    tot_accepted
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(part1(input), 19114);
    assert_eq!(part2(input), 167409079868000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 386787);
    assert_eq!(part2(input), 131029523269531);
}
