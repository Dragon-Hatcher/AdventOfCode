use advent::prelude::*;
use rand::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 25)
}

fn part1(input: &str) -> i64 {
    let mut ids = HashMap::new();
    let mut id = 0;

    let mut edges = Vec::new();

    for (name, conn) in input.lines().map(|l| {
        let (pre, post) = l.split_once(": ").unwrap();
        let post = post.split(" ").collect_vec();
        (pre, post)
    }) {
        ids.entry(name).or_insert_with(|| {
            id += 1;
            id - 1
        });
        for c in conn {
            ids.entry(c).or_insert_with(|| {
                id += 1;
                id - 1
            });
            edges.push((name, c));
        }
    }

    let mut rng = rand::thread_rng();

    loop {
        let mut ids = ids.clone();
        let mut edges = edges.clone();

        edges.shuffle(&mut rng);

        while let Some((a, b)) = edges.pop() {
            let a_id = *ids.get(a).unwrap();
            let b_id = *ids.get(b).unwrap();

            let mut seen: HashMap<i32, i64> = HashMap::new();
            for (_, v) in ids.iter_mut() {
                if *v == b_id {
                    *v = a_id;
                }
                *seen.entry(*v).or_default() += 1;
            }

            edges.retain(|(a, b)| {
                let a_id = *ids.get(a).unwrap();
                let b_id = *ids.get(b).unwrap();
                a_id != b_id
            });

            if seen.len() == 2 {
                if edges.len() == 3 {
                    return seen.values().product();
                }

                break;
            }
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    assert_eq!(part1(input), 54);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 603368);
}
