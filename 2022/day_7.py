from dataclasses import dataclass
import re
import networkx as nx
from typing import Union, Optional, Tuple, List
from rich.pretty import pprint
from rich.console import Console
from rich import print

console = Console()


@dataclass
class Path:
    fullpath: str

    def __post_init__(self):
        try:
            if self.fullpath[-1] != '/':
                self.fullpath = f"{self.fullpath}/"
        except IndexError:
            self.fullpath = '/'
        self.parts = re.findall(r"\w+", self.fullpath)
        if len(self.parts) > 0:
            self.parts.insert(0, "")
        else:
            self.parts = [""]
        self.name = self.parts[-1]
        self.path = f"{'/'.join(self.parts[:-1])}/"

    def __str__(self):
        return f"{self.fullpath}"

    def __hash__(self):
        return hash((self.fullpath, type(self)))

    def __eq__(self, other):
        if hash(other) == hash(self):
            return True
        return False


@dataclass
class File:
    fullpath: str

    def __post_init__(self):
        parts = re.findall(r"[\.\w]+", self.fullpath)
        self.path = f"{'/'.join(parts[:-1])}/"
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
        self.root = Path(fullpath="")
        self.cwd = self.root
        self._tree = nx.DiGraph()
        self._tree.add_node(self.root, objtype="Path", size=0, cumulative_size=0)
        self._stdin_buffer = []
        self.stdout_buffer = []

    def du(self):
        tree = {}
        bfs_tree = {}
        for successor in nx.bfs_tree(self._tree, self.root):
            if isinstance(successor, Path):
                for path, obj in self._tree.edges(successor):
                    if isinstance(obj, Path):
                        self._tree.nodes[path]['cumulative_size'] += self._tree.nodes[obj]['cumulative_size']
                    elif isinstance(obj, File):
                        self._tree.nodes[path]['size'] += self._tree.nodes[obj]['size']
                        self._tree.nodes[path]['cumulative_size'] += self._tree.nodes[obj]['size']

        path_sizes = []
        for node in self._tree:
            if isinstance(node, Path):
                path_sizes.append([self._tree.nodes[node]['cumulative_size'], str(node)])
        path_sizes = list(sorted(path_sizes, key=lambda x: x[0], reverse=True))
        return path_sizes

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

        self._tree.add_node(path, name=path.name, size=0, cumulative_size=0)
        self._tree.add_edge(self.cwd, path)

    def fallocate(self, fullpath: str, size: int):
        fullpath = self.__get_path(fullpath)
        file = File(str(fullpath)[:-1])
        if file in self._tree:
            pprint(f"File {file} already exists")
            return
        self._tree.add_node(file, size=size)
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
                contents[self.cwd].append(
                    f"{self._tree[child]['size']}\t{self._tree[child]['name']}"
                )
        return "\n".join(contents[self.cwd])

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
            if command == 'cd':
                stdin_buffer.append({'command': command, 'args': {'path': stdin[1]}})
            elif command == 'ls':
                continue

        elif re.match(r"\w", line[0]):
            line = line.strip()
            args = line.split(" ")
            if args[0].isdigit():
                stdin_buffer.append({'command': 'fallocate', 'args': {'fullpath': args[1], 'size': int(args[0])}})
            elif args[0].startswith('dir'):
                stdin_buffer.append({'command': 'mkdir', 'args': {'fullpath': args[1]}})

    return stdin_buffer


if __name__ == "__main__":
    sys = System()
    stdin_buffer = stdin_from_file()
    for stdin in stdin_buffer:
        sys.eval(stdin['command'], tuple(stdin['args'].values()))
    print(sys.du())
