from dataclasses import dataclass
from rich.padding import Padding
from rich.segment import Segment
from rich.table import Table, Column
from rich.panel import Panel
from rich.columns import Columns
import re
from typing import Optional
import networkx as nx
from rich.pretty import pprint
from rich.console import Console, ConsoleOptions, RenderResult, Group
from rich import print
from rich.tree import Tree
from rich.text import Text
import datetime

console = Console()


class __FileSystemObject:
    def __init__(self, parts: list) -> None:
        if not parts:
            self.path = ''
            self.name = ''
            self.fullpath = ''
            self.parts = []
            self.pointer = tuple()
            return

        self.name = parts[-1]
        if len(parts) == 1:
            self.path = f"{'/'.join(parts[:-1])}"
        elif len(parts) > 1:
            self.path = f"/{'/'.join(parts[:-1])}"

        self.fullpath = f"{self.path}/{self.name}"
        self.parts = parts
        self.pointer = tuple(parts)

    def __str__(self):
        return f"{'/'.join([''] + self.parts)}"

    def __hash__(self):
        return hash((self.pointer, type(self)))

    def __eq__(self, other):
        if hash(self) == hash(other):
            return True
        else:
            return False

    @classmethod
    def from_string(cls, fullpath: str):
        parts = re.findall(r"[\.\w]+", fullpath)
        if not parts:
            return cls(parts)

        return cls(parts)


class Path(__FileSystemObject):
    def __init__(self, parts):
        super().__init__(parts)
        self.path_size = 0
        self.cumulative_size = 0
        self.name = f"{self.name}/"

    def __str__(self):
        return f"{'/'.join([''] + self.parts + [''])}"

    @classmethod
    def root(cls) -> "Path":
        return cls([])

    @classmethod
    def join(cls, predecessors: "Path", successors: "Path") -> "Path":
        joined = predecessors.parts + successors.parts
        return cls(joined)

class File(__FileSystemObject):
    def __init__(self, parts, size: int = 0):
        super().__init__(parts)
        self.size = size


class System:
    def __init__(self):
        self.disk_space = 70000000
        self.disk_available = self.disk_space
        self.disk_used = 0
        self.root = Path.root()
        self.cwd = self.root
        self.fstree = nx.DiGraph()
        self.fstree.add_node(self.root, objtype=type(Path), size=0, cumulative_size=0)
        self.stdin_buffer = None
        self.stdout_buffer = None

    def __get_path(self, path: str) -> Path:
        if path is None:
            return self.cwd
        if path.startswith('/'):
            return Path.from_string(path)
        elif path.startswith('..'):
            return Path.from_string(self.cwd.path)
        else:
            return Path.join(self.cwd, Path.from_string(path))

    def help(self):
        num_files = 0
        num_paths = 0
        largest_size = 0
        for node in self.fstree.nodes:
            if isinstance(node, File):
                num_files += 1
                if self.fstree.nodes[node]['size'] > largest_size:
                    largest_size += self.fstree.nodes[node]['size']
                    largest_filepath = str(node)
                    largest_filename = node.name
            else:
                num_paths += 1
        tree = Tree(str(self.cwd), guide_style='blue')
        for u, v in self.fstree.out_edges(self.cwd):
            if isinstance(v, Path):
                tree.add(f"[blue]{v}")
            else:
                tree.add(f"[red]{v}")
        help_message = Help(num_files=num_files, num_paths=num_paths, largest_filesize=largest_size, largest_filepath=largest_filepath,
                    largest_filename=largest_filename, cwd=self.cwd, cwd_tree=tree)
        self.stdout_buffer = help_message

    def pwd(self):
        self.stdout_buffer = f"{self.cwd}"

    def mkdir(self, path: str):
        path = self.__get_path(path)
        if path in self.fstree:
            pprint(f"Path {path} already exists.")
            return

        self.fstree.add_node(path, objtype=type(Path), name=path.name, size=0, cumulative_size=0)
        self.fstree.add_edge(self.cwd, path)
        self.stdout_buffer = f"New path created: {path}"

    def fallocate(self, filepath: str, size: int):
        parts = self.__get_path(filepath).parts
        file = File(parts)
        if file in self.fstree:
            pprint(f"File {file} already exists")
            return
        self.fstree.add_node(file, objtype=type(File), size=size)
        self.fstree.add_edge(self.cwd, file)
        self.stdout_buffer = f"New file created: {file}"
        self.disk_used += size
        self.disk_available -= size

    def cd(self, path: str):
        self.cwd = self.__get_path(path)
        self.stdout_buffer = f"Changing path to {path}"

    def ls(self, path: Optional[str] = None):
        path = self.__get_path(path)
        children = []
        for _path, child in self.fstree.out_edges(path):
            children.append(child.name)
        columns = Columns(children, equal=True)
        self.stdout_buffer = columns

        # contents = {self.cwd: []}
        # for path, child in self.fstree.edges(self.cwd):
        #     if isinstance(child, Path):
        #         contents[self.cwd].append(
        #             f"{self.fstree[child]['size']}\t{self.fstree.nodes[child]['cumulative_size']}\t{child}"
        #         )
        #     elif isinstance(child, File):
        #         contents[self.cwd].append(f"{self.fstree.nodes[child]['size']}\t{child}")
        # print("\n".join(contents[self.cwd]))

    def rm(self, obj):
        if Path(obj) in self.fstree:
            obj = Path(obj)
        elif File(obj) in self.fstree:
            obj = File(obj)
        successors = [successor for successor in nx.bfs_tree(self.fstree, obj)]
        for successor in successors:
            self.fstree.remove_node(successor)

    def du(self, path: str = '/', as_tree=False):
        path = self.__get_path(path)
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

    def eval(self, command, *args):
        command = getattr(self, command)
        command(*args)
        console.print(self.stdout_buffer)
        self.stdout_buffer = None

    def interactive(self):
        message = Welcome()
        console.print(message)
        arrow = ":arrow_right: "
        while True:
            now = datetime.datetime.now().strftime("%Y-%m-%d")
            prompt = ' '.join([now, f"[blue]{self.cwd}", arrow])
            stdin = console.input(prompt)
            args = stdin.split(' ')
            command = args[0]
            args = args[1:]
            try:
                self.eval(command, *args)
            except:
                console.print_exception(show_locals=True)



    def __rich_console__(self, console: Console, options: ConsoleOptions) -> RenderResult:
        pass

class Welcome:
    def __init__(self):
        with open('banner.txt', 'r') as f:
            self.banner = f.read()

    def __rich_console__(self, console: Console, options: ConsoleOptions) -> RenderResult:
        banner = Text(self.banner, style='blue', justify='center')
        message = []
        message.append(Text("Entering ineractive mode... There's not too much to see here right now.", style='blue'))
        message.append(Text("This program was created to solve 'Day 7' of the 2022 Advent of Code challenge. This far exceeds the puzzle's requirements, but I thought it would be fun to experiment... Enjoy."))
        command_grid = Table
        yield banner
        for _ in range(2):
            yield Segment.line()
        mode_msg = Text("Interactive Mode\nThere's not much to see here... type 'help' to learn more.", justify='center')
        yield Panel(mode_msg, style='blue')

@dataclass
class Help:
    num_files: int
    num_paths: int
    largest_filename: str
    largest_filepath: str
    largest_filesize: int
    cwd: str
    cwd_tree: Tree

    def __rich_console__(self, console: Console, options: ConsoleOptions) -> RenderResult:
        message = []
        message.append(Text("'Elf Off The Shelf' is a little joke operating system I made while solving 'day 7' of the 2022 'Advent of Code' challenge. The puzzle called to create and navigate a filetree-like and calculate some of its details. Instead, I wrote the full OS for it."))
        message.append(Text("A list of valid commands are below and to your right. They're kind of functional. Give them a shot."))
        details = Table.grid()
        details.add_row(Text(f"This file system has {self.num_files} files among {self.num_paths} paths.\nThe largest file in the system is {self.largest_filename}, whose absolute path is {self.largest_filepath}.\nThe current working directory is {self.cwd}, whose structure looks like:\n"))
        details.add_row(self.cwd_tree)
        commands = Table('Command', 'Description', style='blue', show_edge=False)

        commands.add_row('ls', 'List the contents of the current working directory')
        commands.add_row('cd', 'Change the current directory')
        commands.add_row('pwd', 'List the name of the current directory')
        commands.add_row('rm' ,'Remove a specified file')
        commands.add_row('du', 'List the contents of the filesystem as a filetree with file sizes')
        grid = Table(Column('File System Details'), Column('Command Table'), show_header=False, show_edge=False, style='blue')
        grid.add_row(details, commands)
        grid.show_lines = True
        yield Text('\n').join(message)
        for i in range(3):
            yield Segment.line()
        yield grid


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
    # sys.eval('mkdir', '/home')
    # sys.eval('cd', 'home')
    # sys.eval('fallocate', '/home/test.txt', 125)
    # sys.eval('ls')
    # sys.eval('du', '/', True)

    stdin_buffer = stdin_from_file()
    print(stdin_buffer)
    for stdin in stdin_buffer:
        sys.eval(stdin["command"], *stdin["args"].values())
    sys.interactive()
