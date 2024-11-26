use bfs_builder::IsComplete;
use bon::builder;
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, fmt::Debug, hash::Hash, i64};

#[derive(Debug, Clone)]
pub struct BFSResult<Node> {
    pub node: Node,
    pub steps: i64,
    pub visited: FxHashMap<Node, i64>,
}

#[derive(Debug, Clone)]
pub struct BFSNoResult<Node> {
    pub visited: FxHashMap<Node, i64>,
}

#[builder(finish_fn = try_solve)]
pub fn bfs<Node, NextNodesFn, NextNodesIter, IsGoalFn>(
    start: Node,
    next: NextNodesFn,
    is_goal: IsGoalFn,
    #[builder(default = i64::MAX)] max_iters: i64,
) -> Result<BFSResult<Node>, BFSNoResult<Node>>
where
    Node: Eq + Hash + Clone,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = FxHashMap::default();

    queue.push_back((start.clone(), 0));
    visited.insert(start, 0);

    while let Some((node, steps)) = queue.pop_front() {
        if steps >= max_iters {
            return Err(BFSNoResult { visited });
        }

        if is_goal(&node) {
            return Ok(BFSResult {
                node,
                steps,
                visited,
            });
        }

        for next in (next)(&node) {
            if !visited.contains_key(&next) {
                queue.push_back((next.clone(), steps + 1));
                visited.insert(next, steps + 1);
            }
        }
    }

    Err(BFSNoResult { visited })
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
        match self.try_solve() {
            Ok(info) => info,
            Err(_) => panic!("Expected bfs solution"),
        }
    }

    pub fn finish(self) -> BFSNoResult<Node>
    where
        State: IsComplete,
    {
        match self.try_solve() {
            Ok(_) => panic!("Expected bfs not to finish"),
            Err(info) => info,
        }
    }
}
