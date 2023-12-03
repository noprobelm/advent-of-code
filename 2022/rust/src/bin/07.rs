use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{DfsPostOrder};
use petgraph::Direction;
use std::collections::HashMap;
use std::ops::Index;

fn main() {
    let lines = lines_from_file("../data/7.txt");

    let part_1 = part_1(&lines);
    println!("Part 1: {part_1}");

    let part_2 = part_2(&lines);
    println!("Part 2: {part_2}")
}

fn part_1(lines: &Vec<String>) -> u64 {
    let mut fs: FileSystem = FileSystem::new(0);
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "dir" {
            fs.mkdir(fs.working, split[1]);
        } else if split[0].chars().next().unwrap().is_digit(10) {
            fs.fallocate(
                fs.working,
                split[1],
                split[0].parse::<u64>().expect("Invalid u64!"),
            );
        } else if split[1] == "cd" {
            fs.cd(split[2]);
        }
    }

    let dir_sizes = fs.du();
    let mut puzzle_answer: u64 = 0;
    for (_nx, size) in dir_sizes {
        if size <= 100000 {
            puzzle_answer += size
        }
    }
    puzzle_answer
}

fn part_2(lines: &Vec<String>) -> u64 {
    let mut fs: FileSystem = FileSystem::new(70000000);
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "dir" {
            fs.mkdir(fs.working, split[1]);
        } else if split[0].chars().next().unwrap().is_digit(10) {
            fs.fallocate(
                fs.working,
                split[1],
                split[0].parse::<u64>().expect("Invalid u64!"),
            );
        } else if split[1] == "cd" {
            fs.cd(split[2]);
        }
    }

    let update_size: u64 = 30000000;

    let dir_sizes = fs.du();
    let used_space = dir_sizes.get(&fs.root).unwrap();
    let free_space = fs.capacity - used_space;
    let space_needed = update_size - free_space;
    let answer: u64 = dir_sizes
        .into_values()
        .filter(|x| *x > space_needed)
        .min()
        .unwrap();

    answer
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

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
