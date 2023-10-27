use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use petgraph::Direction;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Node {
    name: String,
    ntype: NodeType,
}

impl Index<NodeIndex> for FileSystem {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.graph[index]
    }
}

impl Node {
    pub fn new(name: &str, ntype: NodeType) -> Self {
        Node {
            name: name.to_string(),
            ntype: ntype,
        }
    }

    pub fn join(&self, target: &str, ntype: NodeType) -> Self {
        let mut split: Vec<&str> = self.name.split("/").collect();
        split.push(target);
        let joined = split.join("/");
        Node::new(&joined, ntype)
    }

    pub fn parent(&self) -> Self {
        let mut split: Vec<&str> = self.name.split("/").collect();
        split.pop();
        let joined = split.join("/");
        Node::new(&joined, NodeType::Directory)
    }

    pub fn root() -> Self {
        Node::new("", NodeType::Directory)
    }
}

pub struct FileSystem {
    pub graph: DiGraph<Node, u64>,
    pub capacity: u64,
    pub root: NodeIndex,
    pub working: NodeIndex,
}

impl FileSystem {
    pub fn new(capacity: u64) -> Self {
        let mut g: DiGraph<Node, u64> = DiGraph::new();
        let root: NodeIndex = g.add_node(Node::root());
        FileSystem {
            graph: g,
            capacity: capacity,
            root: root,
            working: root,
        }
    }

    pub fn get_index(&self, node: Node) -> Option<NodeIndex> {
        for node_idx in self.graph.node_indices() {
            if self.graph[node_idx] == node {
                return Some(node_idx);
            }
        }
        None
    }

    pub fn mkdir(&mut self, path_nx: NodeIndex, name: &str) {
        let node = self[path_nx].join(name, NodeType::Directory);
        let nx = self.graph.add_node(node);
        self.graph.add_edge(path_nx, nx, 0);
    }

    pub fn fallocate(&mut self, path_nx: NodeIndex, name: &str, size: u64) {
        let node = self[path_nx].join(name, NodeType::File);
        let nx = self.graph.add_node(node);
        self.graph.add_edge(path_nx, nx, size);
    }

    pub fn cd(&mut self, name: &str) {
        let path = match name {
            ".." => self.get_index(self[self.working].parent()).unwrap(),
            "/" => self.root,
            _ => self
                .get_index(self[self.working].join(name, NodeType::Directory))
                .unwrap(),
        };
        self.working = path;
    }

    fn reset_dir_weight(&mut self, nx: NodeIndex) {
        let mut neighbors = self
            .graph
            .neighbors_directed(nx, Direction::Incoming)
            .detach();
        while let Some((parent_edge, _)) = neighbors.next(&self.graph) {
            let weight = self.graph.edge_weight_mut(parent_edge).unwrap();
            *weight = 0;
        }
    }
    fn calculate_weights(&mut self) {
        let mut dfs_postorder = DfsPostOrder::new(&self.graph, self.root);
        while let Some(nx) = dfs_postorder.next(&self.graph) {
            if self[nx].ntype == NodeType::Directory {
                self.reset_dir_weight(nx)
            }
            let mut neighbors = self
                .graph
                .neighbors_directed(nx, Direction::Outgoing)
                .detach();
            while let Some((edge_idx, _)) = neighbors.next(&self.graph) {
                let parent = self.get_index(self[nx].parent()).unwrap();
                let child_weight = *self.graph.edge_weight(edge_idx).unwrap();
                if let Some(parent_edge_idx) = self.graph.find_edge(parent, nx) {
                    let weight = self.graph.edge_weight_mut(parent_edge_idx).unwrap();
                    *weight += child_weight;
                }
            }
        }
    }

    pub fn dir_size(&self, node: NodeIndex) -> u64 {
        self.graph
            .neighbors_directed(node, Direction::Outgoing)
            .fold(0, |acc, x| {
                acc + self
                    .graph
                    .edge_weight(self.graph.find_edge(node, x).unwrap())
                    .unwrap()
            })
    }

    pub fn du(&mut self) -> HashMap<NodeIndex, u64> {
        let mut disk_usage: HashMap<NodeIndex, u64> = HashMap::new();
        self.calculate_weights();
        for nx in self.graph.node_indices() {
            match self[nx].ntype {
                NodeType::Directory => {
                    disk_usage.insert(nx, self.dir_size(nx));
                }
                _ => {}
            }
        }
        disk_usage
    }
}
