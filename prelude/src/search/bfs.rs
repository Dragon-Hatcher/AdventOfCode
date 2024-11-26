use bfs_builder::IsComplete;
use bon::builder;
use rustc_hash::FxHashSet;
use std::{collections::VecDeque, fmt::Debug, hash::Hash, i64};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BFSResult<Node> {
    pub node: Node,
    pub steps: i64,
    pub total_visited: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BFSNoResult {
    pub total_visited: i64,
}

#[builder(finish_fn = try_solve)]
pub fn bfs<Node, NextNodesFn, NextNodesIter, IsGoalFn>(
    start: Node,
    next: NextNodesFn,
    is_goal: IsGoalFn,
    #[builder(default = i64::MAX)] max_iters: i64,
) -> Result<BFSResult<Node>, BFSNoResult>
where
    Node: Eq + Hash + Clone,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
{
    let mut queue = VecDeque::new();
    let mut seen = FxHashSet::default();

    queue.push_back((start.clone(), 0));
    seen.insert(start);

    while let Some((node, steps)) = queue.pop_front() {
        if steps >= max_iters {
            return Err(BFSNoResult {
                total_visited: seen.len() as i64,
            });
        }

        if is_goal(&node) {
            return Ok(BFSResult {
                node,
                steps,
                total_visited: seen.len() as i64,
            });
        }

        for next in (next)(&node) {
            if !seen.contains(&next) {
                queue.push_back((next.clone(), steps + 1));
                seen.insert(next);
            }
        }
    }

    Err(BFSNoResult {
        total_visited: seen.len() as i64,
    })
}

impl<Node, NextNodesFn, NextNodesIter, IsGoalFn, State>
    BfsBuilder<Node, NextNodesFn, NextNodesIter, IsGoalFn, State>
where
    Node: Eq + Hash + Clone,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
    State: bfs_builder::State,
{
    pub fn solve(self) -> BFSResult<Node>
    where
        State: IsComplete,
    {
        self.try_solve().unwrap()
    }

    pub fn finish(self) -> BFSNoResult
    where
        Node: Debug,
        State: IsComplete,
    {
        self.try_solve().unwrap_err()
    }
}
