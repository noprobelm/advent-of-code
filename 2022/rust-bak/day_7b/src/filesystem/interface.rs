use crate::filesystem::filesystem::{FileSystem, IndexNode};

#[derive(Debug)]
pub struct WorkingDirectory {
    directory: IndexNode,
}

impl WorkingDirectory {
    fn change_directory(dir: IndexNode) {
        self.directory = dir
    }
}
