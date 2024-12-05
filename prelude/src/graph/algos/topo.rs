use crate::{DiExtraNodeInfo, DiGraph};
use std::hash::Hash;

pub trait TopoSort {
    type Node;

    fn topo_sort(&self) -> Vec<Self::Node>;
}

impl<Node, EdgeWeight, NodeWeight> TopoSort for DiGraph<Node, EdgeWeight, NodeWeight>
where
    Node: Hash + PartialEq + Eq + Clone,
    NodeWeight: Clone,
    EdgeWeight: Clone,
{
    type Node = Node;

    fn topo_sort(&self) -> Vec<Node> {
        fn dfs<Node, EdgeWeight, NodeWeight>(
            at: Node,
            graph: &DiGraph<Node, EdgeWeight, NodeWeight>,
            visited: &mut DiExtraNodeInfo<Node, bool>,
            order: &mut Vec<Node>,
        ) where
            Node: Hash + PartialEq + Eq + Clone,
            NodeWeight: Clone,
            EdgeWeight: Clone,
        {
            *visited.get_mut(at.clone()) = true;

            for (neighbor, _) in graph.outgoing(at.clone()) {
                if *visited.get(&neighbor) {
                    continue;
                }

                dfs(neighbor, graph, visited, order);
            }

            order.push(at);
        }

        let mut order = Vec::new();
        let mut visited = DiExtraNodeInfo::new();

        for node in self.nodes() {
            if *visited.get(node) {
                continue;
            }

            dfs(node.clone(), self, &mut visited, &mut order);
        }

        order
    }
}
