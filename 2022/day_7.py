from dataclasses import dataclass
import re
import networkx as nx
from typing import Union, Optional, Tuple, List
from rich.pretty import pprint
from rich.console import Console
from rich import print
from rich.tree import Tree

console = Console()


@dataclass
class Path:
    fullpath: str

    def __post_init__(self) -> None:
        try:
            if self.fullpath[-1] != "/":
                self.fullpath = f"{self.fullpath}/"
        except IndexError:
            self.fullpath = "/"
        self.parts = re.findall(r"\w+", self.fullpath)
        if len(self.parts) > 0:
            self.parts.insert(0, "")
        else:
            self.parts = [""]
        self.name = f"{self.parts[-1]}/"
        self.path = f"{'/'.join(self.parts[:-1])}/"

    def __str__(self) -> str:
        return f"{self.fullpath}"

    def __hash__(self) -> int:
        return hash((self.fullpath, type(self)))

    def __eq__(self, other: "Path") -> bool:
        if hash(other) == hash(self):
            return True
        return False


@dataclass
class File:
    fullpath: str

    def __post_init__(self):
        parts = re.findall(r"[\.\w]+", self.fullpath)
        self.path = f"/{'/'.join(parts[:-1])}"
        self.name = parts[-1]

    def __str__(self):
        return f"{self.fullpath}{self.name}"

    def __hash__(self):
        return hash((self.fullpath, self.name, type(self)))

    def __eq__(self, other):
        if hash(other) == hash(self):
            return True
        return False


class System:
    def __init__(self):
        self.disk_space = 70000000
        self.disk_used = 0
        self.root = Path(fullpath="")
        self.cwd = self.root
        self._tree = nx.DiGraph()
        self._tree.add_node(self.root, objtype=type(Path), size=0, cumulative_size=0)
        self._stdin_buffer = []
        self.stdout_buffer = []

    def pwd(self):
        return f"{self.cwd}"

    def du(self):
        successors = [successor for successor in nx.bfs_tree(self._tree, self.root)][::-1]

        for successor in successors:
            if isinstance(successor, Path):
                for path, obj in self._tree.out_edges(successor):
                    if isinstance(obj, Path):
                        self._tree.nodes[path]["cumulative_size"] += self._tree.nodes[obj]["cumulative_size"]
                    elif isinstance(obj, File):
                        self._tree.nodes[path]["size"] += self._tree.nodes[obj]["size"]
                        self._tree.nodes[path]["cumulative_size"] += self._tree.nodes[obj]["size"]

        successors = [successor for successor in nx.bfs_successors(self._tree, self.root)]
        trees = {
            self.root: Tree(
                f"{self._tree.nodes[self.root]['cumulative_size']}\t[blue]{self.root.name}", guide_style="blue"
            )
        }
        for node, edges in successors:
            for edge_node in edges:
                if isinstance(edge_node, Path):
                    trees[edge_node] = Tree(
                        f"{self._tree.nodes[edge_node]['cumulative_size']}\t[blue]{edge_node.name}", guide_style="blue"
                    )
                    trees[node].add(trees[edge_node])
                elif isinstance(edge_node, File):
                    trees[node].add(Tree(f"{self._tree.nodes[edge_node]['size']}\t[red]{edge_node.name}"))
        return trees[self.root]

    @property
    def stdin_buffer(self):
        return self._stdin_buffer

    @stdin_buffer.setter
    def stdin_buffer(self, buffer: str):
        self._stdin_buffer = buffer.split(" ")

    def __get_path(self, path):
        if path.startswith("/"):
            return Path(path)
        if path[0].isalpha():
            return Path(f"{self.cwd}{path}/")
        if path == "..":
            return Path(self.cwd.path)

    def mkdir(self, path: str):
        path = self.__get_path(path)
        if path in self._tree:
            pprint(f"Path {path} already exists.")
            return

        self._tree.add_node(path, objtype=type(Path), name=path.name, size=0, cumulative_size=0)
        self._tree.add_edge(self.cwd, path)

    def fallocate(self, fullpath: str, size: int):
        fullpath = self.__get_path(fullpath)
        file = File(str(fullpath)[:-1])
        if file in self._tree:
            pprint(f"File {file} already exists")
            return
        self._tree.add_node(file, objtype=type(File), size=size)
        self._tree.add_edge(self.cwd, file)

    def cd(self, path: str):
        path = self.__get_path(path)
        self.cwd = path

    def ls(self):
        contents = {self.cwd: []}
        for path, child in self._tree.edges(self.cwd):
            if isinstance(child, Path):
                contents[self.cwd].append(
                    f"{self._tree[child]['size']}\t{self._tree[child]['cumulative_size']}\t{self._tree[child]['name']}"
                )
            elif isinstance(child, File):
                contents[self.cwd].append(f"{self._tree[child]['size']}\t{self._tree[child]['name']}")
        return "\n".join(contents[self.cwd])

    def rm(self, obj):
        if Path(obj) in self._tree:
            obj = Path(obj)
        elif File(obj) in self._tree:
            obj = File(obj)
        pprint([successor for successor in nx.bfs_tree(self._tree, obj)])
        successors = [successor for successor in nx.bfs_tree(self._tree, obj)]
        for successor in successors:
            self._tree.remove_node(successor)

    def eval(self, command, args):
        command = getattr(self, command)
        command(*args)


def stdin_from_file():
    with open("day-7-input.txt", "r") as f:
        data = [line for line in f.readlines()]

    stdin_buffer = []
    for num, line in enumerate(data):
        if line.startswith("$"):
            line = line.strip("$")
            line = line.strip()
            stdin = line.split(" ")
            command = stdin[0]
            if command == "cd":
                stdin_buffer.append({"command": command, "args": {"path": stdin[1]}})
            elif command == "ls":
                continue

        elif re.match(r"\w", line[0]):
            line = line.strip()
            args = line.split(" ")
            if args[0].isdigit():
                stdin_buffer.append({"command": "fallocate", "args": {"fullpath": args[1], "size": int(args[0])}})
            elif args[0].startswith("dir"):
                stdin_buffer.append({"command": "mkdir", "args": {"fullpath": args[1]}})

    return stdin_buffer


if __name__ == "__main__":
    sys = System()
    stdin_buffer = stdin_from_file()
    for stdin in stdin_buffer:
        sys.eval(stdin["command"], tuple(stdin["args"].values()))
    paths = sys.du()
    print(paths)
    # answer = sum([path[0] for path in paths])
    # pprint(answer)
#    print(sys.tree)
#    pprint(sys.tree)
#    print(sys.du())
