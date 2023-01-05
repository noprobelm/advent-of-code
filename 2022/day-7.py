from dataclasses import dataclass
import networkx as nx

@dataclass
class Path:
    path: str

    def __post_init__(self):
        self.absolute = '/'.split(self.path)

    def __str__(self):
        return self.path

class System:
    def __init__(self):
        self.tree = nx.DiGraph
        self.tree.add_node(Path('/'))

    def __post_init__(self):
        self.tree.add_node(Path('/'))

    def cd(self):
        pass

@dataclass
class File:
    parent_path: Path
    name: str
    size: int

    def __post_init__(self):
        self.path = f"{self.parent_path}/{self.name}"


@dataclass
class Directory(File):
    name: Path
    parent_path: Path
    files: list[dict[str, int]]
    directories: list[str]
    super().__init__()

    def __post_init__(self):
        self.path = Path(f"{self.parent_path}/{self.name}")



