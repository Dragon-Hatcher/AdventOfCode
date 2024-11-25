use rustc_hash::FxHashSet;
use typed_builder::TypedBuilder;
use std::{collections::VecDeque, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BFSResult<Node> {
    pub node: Node,
    pub steps: i64,
}

#[derive(TypedBuilder)]
pub struct BFS<Node, NextNodesFn, NextNodesIter, IsGoalFn>
where
    Node: Eq + Hash + Clone,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
{
    start: Node,
    next: NextNodesFn,
    is_goal: IsGoalFn,
}

impl<Node, NextNodesFn, NextNodesIter, IsGoalFn> BFS<Node, NextNodesFn, NextNodesIter, IsGoalFn>
where
    Node: Eq + Hash + Clone,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
{
    pub fn try_solve(self) -> Option<BFSResult<Node>> {
        let mut queue = VecDeque::new();
        let mut seen = FxHashSet::default();

        queue.push_back((self.start, 0));

        while let Some((node, steps)) = queue.pop_front() {
            if (self.is_goal)(&node) {
                return Some(BFSResult { node, steps });
            }

            for next in (self.next)(&node) {
                if !seen.contains(&next) {
                    queue.push_back((next.clone(), steps + 1));
                    seen.insert(next);
                }
            }
        }

        None
    }

    pub fn solve(self) -> BFSResult<Node> {
        self.try_solve().unwrap()
    }
}
