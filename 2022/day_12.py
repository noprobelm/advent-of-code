import networkx as nx
from dataclasses import dataclass
from rich import print
from rich.console import Console


@dataclass(frozen=True)
class Node:
    x: int
    y: int


class Maze:
    def __init__(self, mapper):
        self.mapper = mapper
        self.paths = nx.DiGraph()
        for y, row in enumerate(self.mapper):
            for x, node in enumerate(row):
                if node == "S":
                    node = Node(x=x, y=y)
                    self.paths.add_node(node, altitude=ord("a"), start=True)
                    self.mapper[y][x] = "a"
                elif node == "E":
                    node = Node(x=x, y=y)
                    self.paths.add_node(node, altitude=ord("z"), end=True)
                    self.mapper[y][x] = "z"
                else:
                    node = Node(x=x, y=y)
                    self.paths.add_node(node, altitude=ord(self.mapper[y][x]))

        for node in self.paths.nodes:
            self.add_neighbors(node)

    def add_neighbors(self, node):
        if node.x > 0:
            neighbor = Node(node.x - 1, node.y)
            self.paths.add_node(
                neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x])
            )
            if (
                self.paths.nodes[node]["altitude"]
                >= self.paths.nodes[neighbor]["altitude"]
            ):
                self.paths.add_edge(node, neighbor)

            elif (
                self.paths.nodes[neighbor]["altitude"]
                - self.paths.nodes[node]["altitude"]
                == 1
            ):
                self.paths.add_edge(node, neighbor)

        if node.x < len(self.mapper[0]) - 1:
            neighbor = Node(node.x + 1, node.y)
            self.paths.add_node(
                neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x])
            )
            if (
                self.paths.nodes[node]["altitude"]
                >= self.paths.nodes[neighbor]["altitude"]
            ):
                self.paths.add_edge(node, neighbor)
                if node.x == 76 and node.y == 20:
                    print(self.paths.nodes[node])
                    print(self.paths.nodes[neighbor])
                    print(self.mapper[neighbor.y][neighbor.x])

            elif (
                self.paths.nodes[neighbor]["altitude"]
                - self.paths.nodes[node]["altitude"]
                == 1
            ):

                self.paths.add_edge(node, neighbor)
                if node.x == 76 and node.y == 20:
                    print(self.paths.nodes[node])
                    print(self.paths.nodes[neighbor])
        if node.y > 0:
            neighbor = Node(node.x, node.y - 1)
            self.paths.add_node(
                neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x])
            )
            if (
                self.paths.nodes[node]["altitude"]
                >= self.paths.nodes[neighbor]["altitude"]
            ):
                self.paths.add_edge(node, neighbor)

            elif (
                self.paths.nodes[neighbor]["altitude"]
                - self.paths.nodes[node]["altitude"]
                == 1
            ):
                self.paths.add_edge(node, neighbor)

        if node.y < len(self.mapper) - 1:
            neighbor = Node(node.x, node.y + 1)
            self.paths.add_node(
                neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x])
            )
            if (
                self.paths.nodes[node]["altitude"]
                >= self.paths.nodes[neighbor]["altitude"]
            ):
                self.paths.add_edge(node, neighbor)

            elif (
                self.paths.nodes[neighbor]["altitude"]
                - self.paths.nodes[node]["altitude"]
                == 1
            ):
                self.paths.add_edge(node, neighbor)

    @property
    def shortest(self):
        return len(nx.shortest_path(self.paths, self.start, self.end)) - 1

    @property
    def shortest_from_a(self):
        shortest = []
        for node in self.paths.nodes:
            if self.paths.nodes[node]["altitude"] == ord("a"):
                try:
                    shortest.append(len(nx.shortest_path(self.paths, node, self.end)))
                except nx.NetworkXNoPath:
                    pass
        return min(shortest) - 1

    @property
    def start(self):
        return list(nx.get_node_attributes(self.paths, "start").keys())[0]

    @property
    def end(self):
        return list(nx.get_node_attributes(self.paths, "end").keys())[0]


def read(filename):
    with open(filename, "r") as f:
        mapper = [list(row.strip()) for row in f.readlines()]
    return mapper


if __name__ == "__main__":
    console = Console()
    maze = Maze(read("day-12-input.txt"))
    print(maze.shortest)
    print(maze.shortest_from_a)
