use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 24)
}

fn intersection2((p1, v1): (Vector2, Vector2), (p2, v2): (Vector2, Vector2)) -> bool {
    let goal = p1 - p2;
    let det = -v1.x * v2.y - (-v1.y * v2.x);
    let det_inv = 1.0 / (det as f64);

    let m_11 = v2.y as f64 * det_inv;
    let m_21 = v1.y as f64 * det_inv;
    let m_12 = -v2.x as f64 * det_inv;
    let m_22 = -v1.x as f64 * det_inv;

    let t = goal.x as f64 * m_11 + goal.y as f64 * m_12;
    let tp = goal.x as f64 * m_21 + goal.y as f64 * m_22;

    let x = p1.x as f64 + v1.x as f64 * t;
    let y = p1.y as f64 + v1.y as f64 * t;

    const AREA_MIN: f64 = 200000000000000.0;
    const AREA_MAX: f64 = 400000000000000.0;

    x >= AREA_MIN && x <= AREA_MAX && y >= AREA_MIN && y <= AREA_MAX && t >= 0.0 && tp >= 0.0
}

fn part1(input: &str) -> i64 {
    let stones = input
        .lines()
        .map(|l| {
            let (px, py, _, vx, vy, _) = l.nums().tup();
            (Vector2::new(px, py), Vector2::new(vx, vy))
        })
        .collect_vec();

    stones
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| intersection2(*a, *b))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let stones = input
        .lines()
        .map(|l| {
            let (px, py, pz, vx, vy, vz) = l.nums().tup();
            (Vector3::new(px, py, pz), Vector3::new(vx, vy, vz))
        })
        .collect_vec();
    
    eprint!("Solve[{{");
    for (i, stone) in stones.into_iter().take(3).enumerate() {
        let i = i + 1;
        eprint!("px + t{i} * vx == {} + t{i} * {}, ", stone.0.x, stone.1.x);
        eprint!("py + t{i} * vy == {} + t{i} * {}, ", stone.0.y, stone.1.y);
        eprint!("pz + t{i} * vz == {} + t{i} * {}, ", stone.0.z, stone.1.z);
    }
    eprintln!("}},{{px,py,pz,vx,vy,vz,t1,t2,t3}}]");

    // {{px->270392223533307,py->463714142194110,pz->273041846062208,vx->26,vy->-331,vz->53,t1->846337127918,t2->981421067224,t3->573879763083}} 

    270392223533307 + 463714142194110 + 273041846062208

    // let (&a, &b) = stones
    //     .iter()
    //     .tuple_combinations()
    //     .find(|(a, b)| a.1.is_scaling(b.1))
    //     .unwrap();

    // let (p1, v) = a;
    // let (p2, _) = b;

    // let (&l1, &l2) = stones.iter().filter(|s| !s.1.is_scaling(v)).tup();

    // let plane_normal = (p2 - p1).cross(v);
    // let plane_val = plane_normal.dot(p1);

    // let inter_1_t = (plane_val - plane_normal.dot(l1.0)) / plane_normal.dot(l1.1);
    // let inter_2_t = (plane_val - plane_normal.dot(l2.0)) / plane_normal.dot(l2.1);

    // let inter_1 = l1.0 + l1.1 * inter_1_t;
    // let inter_2 = l2.0 + l2.1 * inter_2_t;

    // let diff = inter_2 - inter_1;
    // let t_diff = inter_2_t - inter_1_t;

    // let velocity = Vector3::new(diff.x / t_diff, diff.y / t_diff, diff.z / t_diff);
    // let start_pos = inter_1 - velocity * inter_1_t;

    // start_pos.x + start_pos.y + start_pos.z
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 47);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 14799);
    assert_eq!(part2(input), 1007148211789625);
}
