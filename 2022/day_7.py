from dataclasses import dataclass
import re
import networkx as nx
from typing import Union, Optional, Tuple, List
from rich.pretty import pprint
from rich.console import Console
console = Console()

@dataclass
class Path:
    path: Union["Path", None]
    name: str
    size: int = 0
    cumulative_size: int = 0

    def __post_init__(self):
        if self.path is None:
            self.abspath = ''
        else:
            self.abspath = f"{self.path.abspath}/{self.name}"

        self.children = []

    def __hash__(self):
        return hash((self.abspath, type(self)))

    def __eq__(self, other):
        if self.abspath == other:
            return True
        else:
            return False

    # def __repr__(self):
    #     return self.abspath


@dataclass
class File:
    path: Path
    name: str
    size: int

    def __post_init__(self):
        self.size = int(self.size)
        self.abspath = f"{self.path}/{self.name}"

    def __hash__(self):
        return hash((self.abspath, type(self)))

    def __eq__(self, other):
        if isinstance(other, File) and self.abspath == other:
            return True
        else:
            return False

    def __repr__(self):
        return self.abspath


class System:
    def __init__(self):
        self.root = Path(path=None, name='/')
        self.objects = [self.root]
        self.cwd = self.root
        self._tree = nx.DiGraph()
        self._tree.add_node(self.root, objtype='Path', size=0, cumulative_size=0)
        self.stdout_buffer = []

    @property
    def tree(self):
        successors = [successor for successor in nx.bfs_successors(self._tree, self.root)]
        tree = successors
        return tree

    def cd(self, target):
        pprint(target)
        if target == '/':
            self.cwd = Path(path=None, name='/')
        elif target.startswith('/'):
            target.rstrip('/')
            self.cwd = self.objects[self.objects.index(target[:-1])]
            pprint(self.cwd)
        elif target[0].isalpha():
            self.cwd = Path(path=self.cwd, name=target)
        elif target == '..':
            if self.cwd is not self.root:
                self.cwd = Path(path=self.cwd.path.path, name=self.cwd.path.name)

    def ls(self, squelch: Optional[bool] = True) -> None:
        for obj in self.stdout_buffer:
            if obj[0].isdigit():
                obj = File(path=self.cwd, name=obj[1], size=int(obj[0]))
            else:
                obj = Path(path=self.cwd, name=obj[1])

            if isinstance(obj, Path):
                self.objects.append(obj)
                self._tree.add_edge(obj.path, obj, objtype='Path', size=obj.size, cumulative_size=0)
            elif isinstance(obj, File):
                self.objects.append(obj)
                self._tree.add_edge(obj.path, obj, objtype='File', size=obj.size)

            if squelch is False:
                print(obj)

        self.stdout_buffer = []


if __name__ == '__main__':
    sys = System()

    with open('day-7-input.txt', 'r') as f:
        data = [line for line in f.readlines()]

    stdout_buffer = []
    for line in data:
        if line[0] == '$':
            sys.ls()
            kind = 'stdin'
        else:
            kind = 'stdout'

        if kind == 'stdin':
            line = line[1:].strip().split(' ')
            command = line[0]
            if len(line) > 1:
                arg = line[1]
            if command == 'cd':
                sys.cd(arg)

        elif kind == 'stdout':
            line = line.split(' ')
            sys.stdout_buffer.append([line[0], line[1].strip()])
#    sys.build_tree()
    # for node in sys.tree:
    #     pprint(node)
    pprint(sys.tree)
    sys.cd('/gwlwp/fwdwq/qbnfrhdn/fwdwq/qzgsswr/vhstcbnf/qzgsswr/vqtnpbn/')
    cwd = sys.cwd
    pprint(sys._tree[cwd])
    pprint(sys.tree)


