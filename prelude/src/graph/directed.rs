use rustc_hash::{FxHashMap, FxHashSet};
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
struct NodeData<Node, NodeWeight> {
    weight: NodeWeight,
    outgoing: FxHashSet<Node>,
    incoming: FxHashSet<Node>,
}

#[derive(Debug, Clone)]
pub struct DiGraph<Node, EdgeWeight, NodeWeight = ()> {
    default_node_weight: NodeWeight,
    default_edge_weight: EdgeWeight,

    nodes: FxHashMap<Node, NodeData<Node, NodeWeight>>,
    edges: FxHashMap<(Node, Node), EdgeWeight>,
}

impl<Node, EdgeWeight, NodeWeight> Default for DiGraph<Node, EdgeWeight, NodeWeight>
where
    Node: Hash + PartialEq + Eq + Clone,
    EdgeWeight: Default + Clone,
    NodeWeight: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, EdgeWeight, NodeWeight> DiGraph<Node, EdgeWeight, NodeWeight>
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

    fn ensure_node(&mut self, n: Node) -> &mut NodeData<Node, NodeWeight> {
        self.nodes.entry(n).or_insert_with(|| NodeData {
            weight: self.default_node_weight.clone(),
            outgoing: Default::default(),
            incoming: Default::default(),
        })
    }

    fn ensure_edge(&mut self, from: Node, to: Node) -> &mut EdgeWeight {
        self.edges
            .entry((from, to))
            .or_insert_with(|| self.default_edge_weight.clone())
    }

    fn get_node(&self, node: Node) -> Option<&NodeData<Node, NodeWeight>> {
        self.nodes.get(&node)
    }

    fn get_edge(&self, from: Node, to: Node) -> Option<&EdgeWeight> {
        self.edges.get(&(from, to))
    }

    pub fn add_edge(&mut self, from: Node, to: Node, weight: EdgeWeight) {
        self.ensure_node(from.clone()).outgoing.insert(to.clone());
        self.ensure_node(to.clone()).incoming.insert(from.clone());
        *self.ensure_edge(from, to) = weight;
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> + '_ {
        self.nodes.keys()
    }

    pub fn edges(&self) -> impl Iterator<Item = &(Node, Node)> + '_ {
        self.edges.keys()
    }

    pub fn node_count(&self) -> i64 {
        self.nodes.len() as i64
    }

    pub fn edge_count(&self) -> i64 {
        self.edges.len() as i64
    }

    pub fn outgoing(&self, from: Node) -> impl Iterator<Item = (Node, EdgeWeight)> + '_ {
        self.nodes
            .get(&from)
            .into_iter()
            .flat_map(|data| data.outgoing.iter().cloned())
            .map(move |n| (n.clone(), self.get_edge(from.clone(), n).unwrap().clone()))
    }

    pub fn out_degree(&self, node: Node) -> i64 {
        self.get_node(node).unwrap().outgoing.len() as i64
    }

    pub fn in_degree(&self, node: Node) -> i64 {
        self.get_node(node).unwrap().incoming.len() as i64
    }
}

pub struct DiExtraEdgeInfo<Node, Info> {
    default_info: Info,
    infos: FxHashMap<(Node, Node), Info>,
}

impl<Node, Info> Default for DiExtraEdgeInfo<Node, Info>
where
    Node: Hash + PartialEq + Eq,
    Info: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, Info> DiExtraEdgeInfo<Node, Info>
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

    pub fn get(&self, from: Node, to: Node) -> &Info {
        self.infos.get(&(from, to)).unwrap_or(&self.default_info)
    }

    pub fn get_mut(&mut self, from: Node, to: Node) -> &mut Info {
        self.infos
            .entry((from, to))
            .or_insert_with(|| self.default_info.clone())
    }
}

pub struct DiExtraNodeInfo<Node, Info> {
    default_info: Info,
    infos: FxHashMap<Node, Info>,
}

impl<Node, Info> Default for DiExtraNodeInfo<Node, Info>
where
    Node: Hash + PartialEq + Eq,
    Info: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, Info> DiExtraNodeInfo<Node, Info>
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

    pub fn get(&self, node: &Node) -> &Info {
        self.infos.get(node).unwrap_or(&self.default_info)
    }

    pub fn get_mut(&mut self, node: Node) -> &mut Info {
        self.infos
            .entry(node)
            .or_insert_with(|| self.default_info.clone())
    }
}
