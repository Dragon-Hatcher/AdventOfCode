use rustc_hash::FxHashMap;
use slotmap::{new_key_type, SlotMap};
use std::{hash::Hash, ops::Index};

new_key_type! { pub struct NodeIndex; }
new_key_type! { pub struct EdgeIndex; }

#[derive(Debug)]
pub struct NodeData<Node> {
    val: Node,
    index: NodeIndex,
    edges: FxHashMap<NodeIndex, EdgeIndex>,
}

impl<Node> NodeData<Node> {
    pub fn value(&self) -> &Node {
        &self.val
    }

    pub fn index(&self) -> NodeIndex {
        self.index
    }
}

#[derive(Debug)]
pub struct EdgeData<Weight> {
    index: EdgeIndex,
    ends: (NodeIndex, NodeIndex),
    weight: Weight,
}

impl<Weight> EdgeData<Weight> {
    pub fn index(&self) -> EdgeIndex {
        self.index
    }

    pub fn ends(&self) -> (NodeIndex, NodeIndex) {
        self.ends
    }

    pub fn other_end(&self, end: NodeIndex) -> NodeIndex {
        if self.ends.0 == end {
            self.ends.1
        } else {
            self.ends.0
        }
    }

    pub fn weight(&self) -> &Weight {
        &self.weight
    }
}

pub struct UnGraph<Node, EdgeWeight> {
    nodes: SlotMap<NodeIndex, NodeData<Node>>,
    node_indices: FxHashMap<Node, NodeIndex>,
    edges: SlotMap<EdgeIndex, EdgeData<EdgeWeight>>,
}

impl<Node, EdgeWeight> UnGraph<Node, EdgeWeight>
where
    Node: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        UnGraph {
            nodes: SlotMap::with_key(),
            node_indices: FxHashMap::default(),
            edges: SlotMap::with_key(),
        }
    }

    pub fn new_node(&mut self, val: Node) -> NodeIndex {
        let index = self.nodes.insert_with_key(|index| NodeData {
            val: val.clone(),
            index,
            edges: FxHashMap::default(),
        });
        self.node_indices.insert(val, index);
        index
    }

    pub fn get_node_index(&self, node: &Node) -> Option<NodeIndex> {
        self.node_indices.get(node).copied()
    }

    pub fn node_index(&self, node: &Node) -> NodeIndex {
        self.get_node_index(node).unwrap()
    }

    pub fn ni_or_insert(&mut self, node: Node) -> NodeIndex {
        self.get_node_index(&node)
            .unwrap_or_else(|| self.new_node(node))
    }

    pub fn get_node(&self, index: NodeIndex) -> Option<&NodeData<Node>> {
        self.nodes.get(index)
    }

    pub fn new_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: EdgeWeight) -> EdgeIndex {
        let ei = self.edges.insert_with_key(|index| EdgeData {
            index,
            ends: (from, to),
            weight,
        });

        self.nodes[from].edges.insert(to, ei);
        self.nodes[to].edges.insert(from, ei);

        ei
    }

    pub fn get_edge(&self, index: EdgeIndex) -> Option<&EdgeData<EdgeWeight>> {
        self.edges.get(index)
    }

    pub fn neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self[node].edges.keys().copied()
    }

    pub fn outgoing(&self, node: NodeIndex) -> impl Iterator<Item = &EdgeData<EdgeWeight>> + '_ {
        self[node].edges.values().map(|&ei| &self[ei])
    }
}

impl<Node, EdgeWeight> Index<NodeIndex> for UnGraph<Node, EdgeWeight>
where
    Node: Eq + Hash + Clone,
{
    type Output = NodeData<Node>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.get_node(index).unwrap()
    }
}

impl<Node, EdgeWeight> Index<EdgeIndex> for UnGraph<Node, EdgeWeight>
where
    Node: Eq + Hash + Clone,
{
    type Output = EdgeData<EdgeWeight>;

    fn index(&self, index: EdgeIndex) -> &Self::Output {
        self.get_edge(index).unwrap()
    }
}

impl<Node, EdgeWeight> Default for UnGraph<Node, EdgeWeight>
where
    Node: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
