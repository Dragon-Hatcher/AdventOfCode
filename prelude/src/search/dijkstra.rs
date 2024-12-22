use rustc_hash::FxHashMap;
use std::{fmt::Debug, hash::Hash, i64};

use crate::MinBinaryHeap;

#[derive(Debug, Clone)]
pub struct DijResult<Node> {
    pub stop_node: Option<Node>,
    pub dists: FxHashMap<Node, i64>,
}

pub struct DijEmpty;

pub struct DijBuilder<Node, NextNodesFn, IsGoalFn> {
    start: Node,
    next: NextNodesFn,
    is_goal: IsGoalFn,
    max_iters: i64,
}

pub fn dijkstra() -> DijBuilder<DijEmpty, DijEmpty, DijEmpty> {
    DijBuilder {
        start: DijEmpty,
        next: DijEmpty,
        is_goal: DijEmpty,
        max_iters: i64::MAX,
    }
}

impl<NextNodesFn, IsGoalFn> DijBuilder<DijEmpty, NextNodesFn, IsGoalFn> {
    pub fn start<Node>(self, start: Node) -> DijBuilder<Node, NextNodesFn, IsGoalFn> {
        DijBuilder {
            start,
            next: self.next,
            is_goal: self.is_goal,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, IsGoalFn> DijBuilder<Node, DijEmpty, IsGoalFn> {
    pub fn next<NextNodesFn, NextNodesIter>(
        self,
        next: NextNodesFn,
    ) -> DijBuilder<Node, NextNodesFn, IsGoalFn>
    where
        NextNodesFn: Fn(&Node) -> NextNodesIter,
        NextNodesIter: IntoIterator<Item = (Node, i64)>,
    {
        DijBuilder {
            start: self.start,
            next,
            is_goal: self.is_goal,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, NextNodesFn> DijBuilder<Node, NextNodesFn, DijEmpty> {
    pub fn is_goal<IsGoalFn>(self, is_goal: IsGoalFn) -> DijBuilder<Node, NextNodesFn, IsGoalFn>
    where
        IsGoalFn: Fn(&Node) -> bool,
    {
        DijBuilder {
            start: self.start,
            next: self.next,
            is_goal,
            max_iters: self.max_iters,
        }
    }

    pub fn goal(self, goal: Node) -> DijBuilder<Node, NextNodesFn, impl Fn(&Node) -> bool>
    where
        Node: Eq,
    {
        DijBuilder {
            start: self.start,
            next: self.next,
            is_goal: move |n: &_| n == &goal,
            max_iters: self.max_iters,
        }
    }

    pub fn no_goal(self) -> DijBuilder<Node, NextNodesFn, impl Fn(&Node) -> bool> {
        DijBuilder {
            start: self.start,
            next: self.next,
            is_goal: move |_: &_| false,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, NextNodesFn, IsGoalFn> DijBuilder<Node, NextNodesFn, IsGoalFn> {
    pub fn max_iters(self, max_iters: i64) -> Self {
        Self { max_iters, ..self }
    }
}

impl<Node, NextNodesFn, NextNodesIter, IsGoalFn> DijBuilder<Node, NextNodesFn, IsGoalFn>
where
    Node: Clone + Eq + Hash + Ord,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = (Node, i64)>,
    IsGoalFn: Fn(&Node) -> bool,
{
    pub fn search(self) -> DijResult<Node> {
        let mut queue = MinBinaryHeap::new_min();
        let mut dists = FxHashMap::default();

        queue.push((0, self.start.clone()));
        dists.insert(self.start, 0i64);

        while let Some((dist, next)) = queue.pop() {
            let my_dist = dists[&next];

            // This is a copy of a node we have already seen.
            if my_dist != dist {
                continue;
            }

            if (self.is_goal)(&next) {
                return DijResult {
                    stop_node: Some(next),
                    dists,
                };
            }

            for (neighbor, cost) in (self.next)(&next) {
                let neighbor_dist = dists.get(&neighbor).copied().unwrap_or(i64::MAX);
                let new_dist = my_dist + cost;

                if new_dist < neighbor_dist {
                    dists.insert(neighbor.clone(), new_dist);
                    queue.push((new_dist, neighbor));
                }
            }
        }

        DijResult {
            stop_node: None,
            dists,
        }
    }
}
