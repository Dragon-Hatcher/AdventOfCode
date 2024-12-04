use rustc_hash::{FxHashMap, FxHashSet};
use slotmap::{new_key_type, SlotMap};
use std::{hash::Hash, ops::Index};

// new_key_type! { pub struct NodeIndex; }
// new_key_type! { pub struct EdgeIndex; }

// #[derive(Debug)]
// pub struct NodeData<Node> {
//     val: Node,
//     index: NodeIndex,
//     edges: FxHashMap<NodeIndex, EdgeIndex>,
// }

// impl<Node> NodeData<Node> {
//     pub fn value(&self) -> &Node {
//         &self.val
//     }

//     pub fn index(&self) -> NodeIndex {
//         self.index
//     }
// }

// #[derive(Debug)]
// pub struct EdgeData<Weight> {
//     index: EdgeIndex,
//     ends: (NodeIndex, NodeIndex),
//     weight: Weight,
// }

// impl<Weight> EdgeData<Weight> {
//     pub fn index(&self) -> EdgeIndex {
//         self.index
//     }

//     pub fn ends(&self) -> (NodeIndex, NodeIndex) {
//         self.ends
//     }

//     pub fn other_end(&self, end: NodeIndex) -> NodeIndex {
//         if self.ends.0 == end {
//             self.ends.1
//         } else {
//             self.ends.0
//         }
//     }

//     pub fn weight(&self) -> &Weight {
//         &self.weight
//     }
// }

#[derive(Debug, Clone)]
pub struct UnGraph<Node, EdgeWeight, NodeWeight = ()> {
    default_node_weight: NodeWeight,
    default_edge_weight: EdgeWeight,

    nodes: FxHashMap<Node, (NodeWeight, FxHashSet<Node>)>,
    edges: FxHashMap<(Node, Node), EdgeWeight>,
}

impl<Node, EdgeWeight, NodeWeight> UnGraph<Node, EdgeWeight, NodeWeight>
where
    Node: Hash + PartialEq + Eq + Clone,
    NodeWeight: Clone,
    EdgeWeight: Clone,
{
    pub fn new_with_default(
        default_edge_weight: EdgeWeight,
        default_node_weight: NodeWeight,
    ) -> Self {
        Self {
            default_node_weight,
            default_edge_weight,
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn new() -> Self
    where
        EdgeWeight: Default,
        NodeWeight: Default,
    {
        Self::new_with_default(Default::default(), Default::default())
    }

    fn ensure_node(&mut self, n: Node) -> &mut (NodeWeight, FxHashSet<Node>) {
        self.nodes
            .entry(n)
            .or_insert_with(|| (self.default_node_weight.clone(), FxHashSet::default()))
    }

    fn get_node(&self, n: &Node) -> Option<&(NodeWeight, FxHashSet<Node>)> {
        self.nodes.get(n)
    }

    fn ensure_edge(&mut self, a: Node, b: Node) -> &mut EdgeWeight {
        let key = (b, a);
        if self.edges.contains_key(&key) {
            self.edges.get_mut(&key).unwrap();
        }

        let key = (key.1, key.0);
        self.edges
            .entry(key)
            .or_insert_with(|| self.default_edge_weight.clone())
    }

    fn get_edge(&self, a: Node, b: Node) -> Option<&EdgeWeight> {
        self.edges
            .get(&(a.clone(), b.clone()))
            .or_else(|| self.edges.get(&(b, a)))
    }

    pub fn add_edge(&mut self, a: Node, b: Node, weight: EdgeWeight) {
        self.ensure_node(a.clone()).1.insert(b.clone());
        self.ensure_node(b.clone()).1.insert(a.clone());
        *self.ensure_edge(a, b) = weight;
    }

    pub fn outgoing(&self, from: Node) -> impl Iterator<Item = (Node, EdgeWeight)> + '_ {
        self.nodes
            .get(&from)
            .into_iter()
            .flat_map(|(_, neighbors)| neighbors.iter().cloned())
            .map(move |n| (n.clone(), self.get_edge(from.clone(), n).unwrap().clone()))
    }
}

pub struct ExtraEdgeInfo<Node, Info> {
    default_info: Info,
    infos: FxHashMap<(Node, Node), Info>,
}

impl<Node, Info> ExtraEdgeInfo<Node, Info>
where
    Node: Hash + PartialEq + Eq,
    Info: Clone,
{
    pub fn new_with_default(info: Info) -> Self {
        Self {
            default_info: info,
            infos: Default::default(),
        }
    }

    pub fn new() -> Self
    where
        Info: Default,
    {
        Self::new_with_default(Default::default())
    }

    pub fn get(&mut self, a: Node, b: Node) -> &Info {
        let key = (a, b);
        if self.infos.contains_key(&key) {
            self.infos.get(&key).unwrap();
        }

        let key = (key.1, key.0);
        self.infos
            .entry(key)
            .or_insert_with(|| self.default_info.clone())
    }

    pub fn get_mut(&mut self, a: Node, b: Node) -> &mut Info {
        let key = (a, b);
        if self.infos.contains_key(&key) {
            self.infos.get_mut(&key).unwrap();
        }

        let key = (key.1, key.0);
        self.infos
            .entry(key)
            .or_insert_with(|| self.default_info.clone())
    }
}

// impl<Node, EdgeWeight> UnGraph<Node, EdgeWeight>
// where
//     Node: Eq + Hash + Clone,
// {
//     pub fn new() -> Self {
//         UnGraph {
//             nodes: SlotMap::with_key(),
//             node_indices: FxHashMap::default(),
//             edges: SlotMap::with_key(),
//         }
//     }

//     pub fn new_node(&mut self, val: Node) -> NodeIndex {
//         let index = self.nodes.insert_with_key(|index| NodeData {
//             val: val.clone(),
//             index,
//             edges: FxHashMap::default(),
//         });
//         self.node_indices.insert(val, index);
//         index
//     }

//     pub fn get_node_index(&self, node: &Node) -> Option<NodeIndex> {
//         self.node_indices.get(node).copied()
//     }

//     pub fn node_index(&self, node: &Node) -> NodeIndex {
//         self.get_node_index(node).unwrap()
//     }

//     pub fn ni_or_insert(&mut self, node: Node) -> NodeIndex {
//         self.get_node_index(&node)
//             .unwrap_or_else(|| self.new_node(node))
//     }

//     pub fn get_node(&self, index: NodeIndex) -> Option<&NodeData<Node>> {
//         self.nodes.get(index)
//     }

//     pub fn new_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: EdgeWeight) -> EdgeIndex {
//         let ei = self.edges.insert_with_key(|index| EdgeData {
//             index,
//             ends: (from, to),
//             weight,
//         });

//         self.nodes[from].edges.insert(to, ei);
//         self.nodes[to].edges.insert(from, ei);

//         ei
//     }

//     pub fn get_edge(&self, index: EdgeIndex) -> Option<&EdgeData<EdgeWeight>> {
//         self.edges.get(index)
//     }

//     pub fn neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
//         self[node].edges.keys().copied()
//     }

//     pub fn outgoing(&self, node: NodeIndex) -> impl Iterator<Item = &EdgeData<EdgeWeight>> + '_ {
//         self[node].edges.values().map(|&ei| &self[ei])
//     }
// }

// impl<Node, EdgeWeight> Index<NodeIndex> for UnGraph<Node, EdgeWeight>
// where
//     Node: Eq + Hash + Clone,
// {
//     type Output = NodeData<Node>;

//     fn index(&self, index: NodeIndex) -> &Self::Output {
//         self.get_node(index).unwrap()
//     }
// }

// impl<Node, EdgeWeight> Index<EdgeIndex> for UnGraph<Node, EdgeWeight>
// where
//     Node: Eq + Hash + Clone,
// {
//     type Output = EdgeData<EdgeWeight>;

//     fn index(&self, index: EdgeIndex) -> &Self::Output {
//         self.get_edge(index).unwrap()
//     }
// }

// impl<Node, EdgeWeight> Default for UnGraph<Node, EdgeWeight>
// where
//     Node: Eq + Hash + Clone,
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }
