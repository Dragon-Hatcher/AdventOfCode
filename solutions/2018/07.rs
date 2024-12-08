use advent::prelude::*;
use std::cmp::Reverse;

fn default_input() -> &'static str {
    include_input!("wait=60, workers=5\n\n" / 2018 / 07)
}

type Step = char;

fn parse(input: &str) -> DiGraph<Step, ()> {
    let mut graph = DiGraph::new();
    for line in input.lines() {
        let pre_req = line.chars().nth(5).unwrap();
        let post_req = line.chars().nth(36).unwrap();
        graph.add_edge(pre_req, post_req, ());
    }
    graph
}

fn part1(input: &str) -> String {
    let (_, input) = input.sections().tup();
    let graph = parse(input);

    let mut in_degree: DiExtraNodeInfo<Step, i64> = DiExtraNodeInfo::new();
    for &node in graph.nodes() {
        *in_degree.get_mut(node) = graph.in_degree(node);
    }

    let mut order = Vec::new();
    let mut zero_in_degree = graph
        .nodes()
        .copied()
        .filter(|n| *in_degree.get(n) == 0)
        .collect_vec();
    zero_in_degree.sort_by_key(|n| Reverse(*n));

    while let Some(next) = zero_in_degree.pop() {
        order.push(next);

        for (neighbor, _) in graph.outgoing(next) {
            *in_degree.get_mut(neighbor) -= 1;
            if *in_degree.get(&neighbor) == 0 {
                zero_in_degree.push(neighbor);
            }
        }

        zero_in_degree.sort_by_key(|n| Reverse(*n));
    }

    order.into_iter().collect()
}

fn part2(input: &str) -> i64 {
    let (config, input) = input.sections().tup();
    let (wait_time, max_workers) = config.nums().tup();
    let graph = parse(input);

    let mut in_degree: DiExtraNodeInfo<Step, i64> = DiExtraNodeInfo::new();
    for &node in graph.nodes() {
        *in_degree.get_mut(node) = graph.in_degree(node);
    }

    let mut tasks_to_do: BinaryHeap<char> = graph
        .nodes()
        .copied()
        .filter(|n| *in_degree.get(n) == 0)
        .collect();
    let mut completion_times = BinaryHeap::new_min();

    let mut workers_left = max_workers;
    let mut current_min = 0;

    while !(tasks_to_do.is_empty() && completion_times.is_empty()) {
        if completion_times.peek().map(|(t, _)| t) == Some(&current_min) {
            // A task is now completed.
            let (_, complete) = completion_times.pop().unwrap();
            workers_left += 1;

            for (neighbor, _) in graph.outgoing(complete) {
                *in_degree.get_mut(neighbor) -= 1;
                if *in_degree.get(&neighbor) == 0 {
                    tasks_to_do.push(neighbor);
                }
            }
        } else if workers_left > 0 && !tasks_to_do.is_empty() {
            // Assign the next task in the queue to a worker.
            workers_left -= 1;
            let todo = tasks_to_do.pop().unwrap();
            completion_times.push((
                current_min + wait_time + (todo as i64 - 'A' as i64) + 1,
                todo,
            ));
        } else {
            // Jump forward to the next minute
            current_min = completion_times.peek().unwrap().0;
        }
    }

    current_min
}

fn main() {
    advent::new(2018, 7, default_input)
        .part1(part1)
        .part2(part2)
        .cli();
}

#[test]
fn example() {
    let input = "wait=0, workers=2

Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!(part1(input), "CABDFE");
    assert_eq!(part2(input), 15);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "JRHSBCKUTVWDQAIGYOPXMFNZEL");
    assert_eq!(part2(input), 975);
}
