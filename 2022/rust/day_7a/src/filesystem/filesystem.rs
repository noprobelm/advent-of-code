use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use uuid::Uuid;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum NodeType {
    File(u32),
    Directory,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct IndexNode {
    pub id: Uuid,
    pub name: String,
    node_type: NodeType,
}

impl IndexNode {
    pub fn new(name: &str, node_type: NodeType) -> IndexNode {
        let id = Uuid::new_v4();
        IndexNode {
            id: id,
            name.clone(),
            node_type: node_type,
        }
    }
}

#[derive(Debug)]
pub struct FileSystem {
    pub root: IndexNode,
    pub working: IndexNode,
    pub map: HashMap<Uuid, IndexNode>,
    pub alist: HashMap<IndexNode, VecDeque<IndexNode>>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let root = IndexNode {
            id: Uuid::new_v4(),
            name: "",
            node_type: NodeType::Directory,
        };

        let map = HashMap::from([(root.id, root)]);
        let alist = HashMap::from([(root, VecDeque::new())]);

        FileSystem {
            root: root,
            working: root,
            map: map,
            alist: alist,
        }
    }

    pub fn add_file(&mut self, parent_node: IndexNode, name: &str, size: u32) -> IndexNode {
        let id = Uuid::new_v4();
        let file = IndexNode {
            id: id,
            name: name.clone(),
            node_type: NodeType::File(size),
        };

        self.map.insert(id, file);
        self.alist.insert(file, VecDeque::new());

        file
    }

    pub fn add_directory(&mut self, parent_node: IndexNode, name: &str) -> IndexNode {
        let id = Uuid::new_v4();
        let directory = IndexNode {
            id: id,
            name: name.clone(),
            node_type: NodeType::Directory,
        };

        self.map.insert(id, directory);
        self.alist.insert(directory, VecDeque::new());

        directory
    }

    pub fn get(&self, node: IndexNode) {

    }
}
