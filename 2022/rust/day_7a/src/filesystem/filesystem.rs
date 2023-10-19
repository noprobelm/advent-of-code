use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub name: String,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node { name: name }
    }

    pub fn root() -> Self {
        Node {
            name: "".to_string(),
        }
    }

    pub fn join(&self, target: Node) -> Self {
        let mut parts = self.parts();
        parts.push(target.name.as_str());
        Node::new(parts.join("/"))
    }

    pub fn parent(&self) -> Self {
        let mut parts = self.parts();
        parts.pop();
        let parent = parts.join("/");
        Node::new(parent)
    }

    pub fn parts(&self) -> Vec<&str> {
        self.name.split("/").collect::<Vec<&str>>()
    }
}

#[derive(Debug)]
pub struct FileSystem {
    alist: HashMap<Node, Vec<Node>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut alist: HashMap<Node, Vec<Node>> = HashMap::new();
        let mut v: Vec<Node> = Vec::new();
        alist.insert(Node::root(), v);
        FileSystem { alist: alist }
    }

    pub fn insert(&mut self, parent: Node, target: Node) {
        let mut v: Vec<Node> = Vec::new();
        self.alist.insert(target, v);
        self.alist
            .entry(parent)
            .and_modify(|v| v.push(target.clone()));
    }
}
