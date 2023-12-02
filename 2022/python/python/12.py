import networkx as nx
from dataclasses import dataclass
from rich import print


def main():
    maze = Maze(read("../../data/12.txt"))
    print(f"Part 1: {maze.shortest}")
    print(f"Part 2: {maze.shortest_from_altitude('a')}")


@dataclass(frozen=True)
class Node:
    """Hashable object to be used when referencing nodes in the maze's graph"""

    x: int
    y: int


class Maze:
    def __init__(self, mapper: list[list[str]]) -> None:
        """
        Accepts a 'mapper' (list of list of strings) representing the altitude
        of each node along our maze. Our instance variable 'paths' acts as the
        graph. First, add each node to the graph. Then check for valid
        neighbors and add them using add_neighbors method if appropriate.

        - Property 'shortest' will return the shortest path between 'S' and 'E'
        - Method 'shortest_from_altitude' will return the shortest path between
            any specified altitude and 'E'
        """
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
        """
        - A neighbor is valid for traversal if:
            The difference between the subject node and the beighbor is greater
            than '-1' (i.e., the neighbor is at most one unit taller than the
            subject node)

        If a neighbor is deemed as a valid traversal node, add an edge between
        it and the subject node
        """
        evaluations = (
            (Node(node.x - 1, node.y), node.x > 0),
            (Node(node.x + 1, node.y), node.x < len(self.mapper[0]) - 1),
            (Node(node.x, node.y - 1), node.y > 0),
            (Node(node.x, node.y + 1), node.y < len(self.mapper) - 1),
        )
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
        """Returns the shortest path between the maze start and end"""
        return len(nx.shortest_path(self.paths, self.start, self.end)) - 1

    def shortest_from_altitude(self, altitude: str) -> int:
        """Returns the shortest path between any specified altitude and end"""
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
        """Returns the start Node"""
        return list(nx.get_node_attributes(self.paths, "start").keys())[0]

    @property
    def end(self) -> Node:
        """Returns the end Node"""
        return list(nx.get_node_attributes(self.paths, "end").keys())[0]


def read(filename: str) -> list[list[str]]:
    """Reads the puzzle input as a list of list of strings ('mapper')"""
    with open(filename, "r") as f:
        mapper = [list(row.strip()) for row in f.readlines()]
    return mapper


if __name__ == "__main__":
    main()
