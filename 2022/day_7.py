from dataclasses import dataclass
import re
import networkx as nx
from rich.pretty import pprint
from rich.console import Console, ConsoleOptions, RenderResult
from rich import print
from rich.tree import Tree
import datetime

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
        self.disk_available = self.disk_space
        self.disk_used = 0
        self.root = Path(fullpath="")
        self.cwd = self.root
        self.fstree = nx.DiGraph()
        self.fstree.add_node(self.root, objtype=type(Path), size=0, cumulative_size=0)
        self.stdin_buffer = None
        self.stdout_buffer = None

    def __get_path(self, path: str) -> Path:
        if not path or path.startswith("/"):
            return Path(path)
        if path[0].isalpha():
            return Path(f"{self.cwd}{path}/")
        if path == "..":
            return Path(self.cwd.path)

    def pwd(self):
        return f"{self.cwd}"

    def mkdir(self, path: str):
        path = self.__get_path(path)
        if path in self.fstree:
            pprint(f"Path {path} already exists.")
            return

        self.fstree.add_node(path, objtype=type(Path), name=path.name, size=0, cumulative_size=0)
        self.fstree.add_edge(self.cwd, path)

    def fallocate(self, fullpath: str, size: int):
        fullpath = self.__get_path(fullpath)
        file = File(str(fullpath)[:-1])
        if file in self.fstree:
            pprint(f"File {file} already exists")
            return
        self.fstree.add_node(file, objtype=type(File), size=size)
        self.fstree.add_edge(self.cwd, file)
        self.disk_used += size
        self.disk_available -= size

    def cd(self, path: str):
        path = self.__get_path(path)
        self.cwd = path
        print(f"Changed path to {self.cwd}")

    def ls(self):
        contents = {self.cwd: []}
        for path, child in self.fstree.edges(self.cwd):
            if isinstance(child, Path):
                contents[self.cwd].append(
                    f"{self.fstree[child]['size']}\t{self.fstree.nodes[child]['cumulative_size']}\t{child}"
                )
            elif isinstance(child, File):
                contents[self.cwd].append(f"{self.fstree.nodes[child]['size']}\t{child}")
        print("\n".join(contents[self.cwd]))

    def rm(self, obj):
        if Path(obj) in self.fstree:
            obj = Path(obj)
        elif File(obj) in self.fstree:
            obj = File(obj)
        successors = [successor for successor in nx.bfs_tree(self.fstree, obj)]
        for successor in successors:
            self.fstree.remove_node(successor)

    def du(self, path: str = '', as_tree: bool = False):
        path = self.__get_path('')
        for node in self.fstree.nodes:
            if isinstance(node, Path):
                self.fstree.nodes[node]['size'] = 0
                self.fstree.nodes[node]['cumulative_size'] = 0

        children = [child for child in nx.bfs_tree(self.fstree, path)][::-1]
        for child in children:
            if isinstance(child, Path):
                for _child, _child_obj in self.fstree.out_edges(child):
                    if isinstance(_child_obj, Path):
                        self.fstree.nodes[child]["cumulative_size"] += self.fstree.nodes[_child_obj]["cumulative_size"]
                    elif isinstance(_child_obj, File):
                        self.fstree.nodes[child]["size"] += self.fstree.nodes[_child_obj]["size"]
                        self.fstree.nodes[child]["cumulative_size"] += self.fstree.nodes[_child_obj]["size"]

        if not as_tree:
            stdout = []
            for node in self.fstree.nodes:
                if isinstance(node, Path):
                    stdout.append([self.fstree.nodes[node]['cumulative_size'], f"[blue]{node}[/blue]"])
                elif isinstance(node, File):
                    stdout.append([self.fstree.nodes[node]['size'], f"[red]{node}[/red]"])
            stdout = list(sorted(stdout, key=lambda s: s[0], reverse=True))
            stdout = ['\t'.join([str(output[0]), output[1]]) for output in stdout]
            stdout = '\n'.join(stdout)
        else:
            children = [successor for successor in nx.bfs_successors(self.fstree, path)]
            trees = {
                self.root: Tree(
                    f"{self.fstree.nodes[path]['cumulative_size']}\t[blue]{path}", guide_style="blue"
                )
            }
            for node, edges in children:
                for edge_node in edges:
                    if isinstance(edge_node, Path):
                        trees[edge_node] = Tree(
                            f"{self.fstree.nodes[edge_node]['cumulative_size']}\t[blue]{edge_node.name}", guide_style="blue"
                        )
                        trees[node].add(trees[edge_node])
                    elif isinstance(edge_node, File):
                        trees[node].add(Tree(f"{self.fstree.nodes[edge_node]['size']}\t[red]{edge_node.name}"))
            stdout = trees[self.root]
        self.stdout_buffer = stdout
        return self.stdout_buffer

    def eval(self, command, args):
        command = getattr(self, command)
        command(*args)

    def interactive(self):
        arrow = ":arrow_right: "
        while True:
            now = datetime.datetime.now().strftime("%Y-%m-%d")
            prompt = ' '.join([now, arrow])
            stdin = console.input(prompt)
            args = stdin.split(' ')
            command = args[0]
            args = args[1:]
            if self.stdout_buffer:
                print(self.stdout_buffer)
                self.stdout_buffer = None



    def __rich_console__(self, console: Console, options: ConsoleOptions) -> RenderResult:
        pass


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
    print(stdin_buffer)
    for stdin in stdin_buffer:
        sys.eval(stdin["command"], tuple(stdin["args"].values()))
    paths = sys.du()
    print(paths)
