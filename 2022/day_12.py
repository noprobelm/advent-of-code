import networkx as nx
from dataclasses import dataclass
from rich import print


@dataclass(frozen=True)
class Node:
    x: int
    y: int


class Maze:
    def __init__(self, mapper: list[list[str]]) -> None:
        self.mapper = mapper
        self.paths = nx.DiGraph()
        for y, row in enumerate(self.mapper):
            for x, node in enumerate(row):
                if node == "S":
                    self.paths.add_node(Node(x=x, y=y), altitude=ord("a"), start=True)
                    self.mapper[y][x] = "a"
                elif node == "E":
                    self.paths.add_node(Node(x=x, y=y), altitude=ord("z"), end=True)
                    self.mapper[y][x] = "z"
                else:
                    self.paths.add_node(Node(x=x, y=y), altitude=ord(self.mapper[y][x]))

        for node in self.paths.nodes:
            self.add_neighbors(node)

    def add_neighbors(self, node: Node) -> None:
        evaluations = [
            (Node(node.x - 1, node.y), node.x > 0),
            (Node(node.x + 1, node.y), node.x < len(self.mapper[0]) - 1),
            (Node(node.x, node.y - 1), node.y > 0),
            (Node(node.x, node.y + 1), node.y < len(self.mapper) - 1),
        ]
        for evaluation in evaluations:
            neighbor = evaluation[0]
            if (
                evaluation[1]
                and self.paths.nodes[node]["altitude"]
                - self.paths.nodes[neighbor]["altitude"]
                >= -1
            ):
                self.paths.add_edge(node, neighbor)

    @property
    def shortest(self) -> int:
        return len(nx.shortest_path(self.paths, self.start, self.end)) - 1

    def shortest_from_altitude(self, altitude: str):
        shortest = []
        for node in self.paths.nodes:
            if self.paths.nodes[node]["altitude"] == ord(altitude):
                try:
                    shortest.append(len(nx.shortest_path(self.paths, node, self.end)))
                except nx.NetworkXNoPath:
                    pass
        return min(shortest) - 1

    @property
    def start(self) -> Node:
        return list(nx.get_node_attributes(self.paths, "start").keys())[0]

    @property
    def end(self) -> Node:
        return list(nx.get_node_attributes(self.paths, "end").keys())[0]


def read(filename: str) -> list[list[str]]:
    with open(filename, "r") as f:
        mapper = [list(row.strip()) for row in f.readlines()]
    return mapper


if __name__ == "__main__":
    maze = Maze(read("day-12-input.txt"))
    print(maze.shortest)
    print(maze.shortest_from_altitude("a"))
