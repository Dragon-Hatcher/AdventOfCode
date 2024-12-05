use rustc_hash::{FxHashMap, FxHashSet};
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
pub struct UnGraph<Node, EdgeWeight, NodeWeight = ()> {
    default_node_weight: NodeWeight,
    default_edge_weight: EdgeWeight,

    nodes: FxHashMap<Node, (NodeWeight, FxHashSet<Node>)>,
    edges: FxHashMap<(Node, Node), EdgeWeight>,
}

impl<Node, EdgeWeight, NodeWeight> Default for UnGraph<Node, EdgeWeight, NodeWeight>
where
    Node: Hash + PartialEq + Eq + Clone,
    EdgeWeight: Default + Clone,
    NodeWeight: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
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

    fn ensure_edge(&mut self, a: Node, b: Node) -> &mut EdgeWeight {
        let key = (b, a);
        if self.edges.contains_key(&key) {
            return self.edges.get_mut(&key).unwrap();
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

pub struct UnExtraEdgeInfo<Node, Info> {
    default_info: Info,
    infos: FxHashMap<(Node, Node), Info>,
}

impl<Node, Info> Default for UnExtraEdgeInfo<Node, Info>
where
    Node: Hash + PartialEq + Eq,
    Info: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, Info> UnExtraEdgeInfo<Node, Info>
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

    pub fn get(&self, a: Node, b: Node) -> &Info
    where
        Node: Debug,
        Info: Debug,
    {
        let key = (a, b);
        if self.infos.contains_key(&key) {
            return self.infos.get(&key).unwrap();
        }

        let key = (key.1, key.0);
        if self.infos.contains_key(&key) {
            return self.infos.get(&key).unwrap();
        }

        &self.default_info
    }

    pub fn get_mut(&mut self, a: Node, b: Node) -> &mut Info {
        let key = (a, b);
        if self.infos.contains_key(&key) {
            return self.infos.get_mut(&key).unwrap();
        }

        let key = (key.1, key.0);
        self.infos
            .entry(key)
            .or_insert_with(|| self.default_info.clone())
    }
}
