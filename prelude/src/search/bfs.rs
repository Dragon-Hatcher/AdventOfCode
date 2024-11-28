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

pub struct Empty;

pub struct BfsBuilder<Node, NextNodesFn, IsGoalFn> {
    start: Node,
    next: NextNodesFn,
    is_goal: IsGoalFn,
    max_iters: i64,
}

pub fn bfs() -> BfsBuilder<Empty, Empty, Empty> {
    BfsBuilder {
        start: Empty,
        next: Empty,
        is_goal: Empty,
        max_iters: i64::MAX,
    }
}

impl<NextNodesFn, IsGoalFn> BfsBuilder<Empty, NextNodesFn, IsGoalFn> {
    pub fn start<Node>(self, start: Node) -> BfsBuilder<Node, NextNodesFn, IsGoalFn> {
        BfsBuilder {
            start,
            next: self.next,
            is_goal: self.is_goal,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, IsGoalFn> BfsBuilder<Node, Empty, IsGoalFn> {
    pub fn next<NextNodesFn, NextNodesIter>(
        self,
        next: NextNodesFn,
    ) -> BfsBuilder<Node, NextNodesFn, IsGoalFn>
    where
        NextNodesFn: Fn(&Node) -> NextNodesIter,
        NextNodesIter: IntoIterator<Item = Node>,
    {
        BfsBuilder {
            start: self.start,
            next,
            is_goal: self.is_goal,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, NextNodesFn> BfsBuilder<Node, NextNodesFn, Empty> {
    pub fn is_goal<IsGoalFn>(self, is_goal: IsGoalFn) -> BfsBuilder<Node, NextNodesFn, IsGoalFn>
    where
        IsGoalFn: Fn(&Node) -> bool,
    {
        BfsBuilder {
            start: self.start,
            next: self.next,
            is_goal,
            max_iters: self.max_iters,
        }
    }

    pub fn goal(self, goal: Node) -> BfsBuilder<Node, NextNodesFn, impl Fn(&Node) -> bool>
    where
        Node: Eq,
    {
        BfsBuilder {
            start: self.start,
            next: self.next,
            is_goal: move |n: &_| n == &goal,
            max_iters: self.max_iters,
        }
    }

    pub fn no_goal(self) -> BfsBuilder<Node, NextNodesFn, impl Fn(&Node) -> bool> {
        BfsBuilder {
            start: self.start,
            next: self.next,
            is_goal: move |_: &_| false,
            max_iters: self.max_iters,
        }
    }
}

impl<Node, NextNodesFn, IsGoalFn> BfsBuilder<Node, NextNodesFn, IsGoalFn> {
    pub fn max_iters(self, max_iters: i64) -> Self {
        Self { max_iters, ..self }
    }
}

impl<Node, NextNodesFn, NextNodesIter, IsGoalFn> BfsBuilder<Node, NextNodesFn, IsGoalFn>
where
    Node: Clone + Eq + Hash,
    NextNodesFn: Fn(&Node) -> NextNodesIter,
    NextNodesIter: IntoIterator<Item = Node>,
    IsGoalFn: Fn(&Node) -> bool,
{
    pub fn try_solve(self) -> Result<BFSResult<Node>, BFSNoResult<Node>> {
        let mut queue = VecDeque::new();
        let mut visited = FxHashMap::default();

        queue.push_back((self.start.clone(), 0));
        visited.insert(self.start, 0);

        while let Some((node, steps)) = queue.pop_front() {
            if steps >=self.max_iters {
                return Err(BFSNoResult { visited });
            }

            if (self.is_goal)(&node) {
                return Ok(BFSResult {
                    node,
                    steps,
                    visited,
                });
            }

            for next in (self.next)(&node) {
                visited.entry(next.clone()).or_insert_with(|| {
                    queue.push_back((next, steps + 1));
                    steps + 1
                });
            }
        }

        Err(BFSNoResult { visited })
    }

    pub fn solve(self) -> BFSResult<Node> {
        match self.try_solve() {
            Ok(info) => info,
            Err(_) => panic!("Expected bfs solution"),
        }
    }

    pub fn finish(self) -> BFSNoResult<Node> {
        match self.try_solve() {
            Ok(_) => panic!("Expected bfs not to finish"),
            Err(info) => info,
        }
    }
}
