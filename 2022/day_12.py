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
        self.G = nx.DiGraph()
        for y, row in enumerate(self.mapper):
            for x, node in enumerate(row):
                if node == "S":
                    node = Node(x=x, y=y)
                    self.G.add_node(node, altitude=ord("a"), start=True)
                    self.mapper[y][x] = "a"
                elif node == "E":
                    node = Node(x=x, y=y)
                    self.G.add_node(node, altitude=ord("z"), end=True)
                    self.mapper[y][x] = "z"
                else:
                    node = Node(x=x, y=y)
                    self.G.add_node(node, altitude=ord(self.mapper[y][x]))

        for node in self.G.nodes:
            self.add_neighbors(node)

    def add_neighbors(self, node):
        if node.x > 0:
            neighbor = Node(node.x - 1, node.y)
            self.G.add_node(neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x]))
            if self.G.nodes[node]["altitude"] >= self.G.nodes[neighbor]["altitude"]:
                self.G.add_edge(node, neighbor)

            elif (
                self.G.nodes[neighbor]["altitude"] - self.G.nodes[node]["altitude"] == 1
            ):
                self.G.add_edge(node, neighbor)

        if node.x < len(self.mapper[0]) - 1:
            neighbor = Node(node.x + 1, node.y)
            self.G.add_node(neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x]))
            if self.G.nodes[node]["altitude"] >= self.G.nodes[neighbor]["altitude"]:
                self.G.add_edge(node, neighbor)
                if node.x == 76 and node.y == 20:
                    print(self.G.nodes[node])
                    print(self.G.nodes[neighbor])
                    print(self.mapper[neighbor.y][neighbor.x])

            elif (
                self.G.nodes[neighbor]["altitude"] - self.G.nodes[node]["altitude"] == 1
            ):

                self.G.add_edge(node, neighbor)
                if node.x == 76 and node.y == 20:
                    print(self.G.nodes[node])
                    print(self.G.nodes[neighbor])
        if node.y > 0:
            neighbor = Node(node.x, node.y - 1)
            self.G.add_node(neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x]))
            if self.G.nodes[node]["altitude"] >= self.G.nodes[neighbor]["altitude"]:
                self.G.add_edge(node, neighbor)

            elif (
                self.G.nodes[neighbor]["altitude"] - self.G.nodes[node]["altitude"] == 1
            ):
                self.G.add_edge(node, neighbor)

        if node.y < len(self.mapper) - 1:
            neighbor = Node(node.x, node.y + 1)
            self.G.add_node(neighbor, altitude=ord(self.mapper[neighbor.y][neighbor.x]))
            if self.G.nodes[node]["altitude"] >= self.G.nodes[neighbor]["altitude"]:
                self.G.add_edge(node, neighbor)

            elif (
                self.G.nodes[neighbor]["altitude"] - self.G.nodes[node]["altitude"] == 1
            ):
                self.G.add_edge(node, neighbor)

    @property
    def shortest(self):
        return len(nx.shortest_path(self.G, self.start, self.end)) - 1

    @property
    def shortest_from_a(self):
        shortest = []
        for node in self.G.nodes:
            if self.G.nodes[node]["altitude"] == ord("a"):
                try:
                    shortest.append(len(nx.shortest_path(self.G, node, self.end)))
                except nx.NetworkXNoPath:
                    pass
        return min(shortest) - 1

    @property
    def start(self):
        return list(nx.get_node_attributes(self.G, "start").keys())[0]

    @property
    def end(self):
        return list(nx.get_node_attributes(self.G, "end").keys())[0]


def read(filename):
    with open(filename, "r") as f:
        mapper = [list(row.strip()) for row in f.readlines()]
    return mapper


if __name__ == "__main__":
    console = Console()
    maze = Maze(read("day-12-input.txt"))
    print(maze.shortest)
    print(maze.shortest_from_a)
