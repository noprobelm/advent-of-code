from typing import List, Optional, Sequence
from rich.console import Console
from dataclasses import dataclass
import networkx as nx
console = Console()


@dataclass(frozen=True, order=True)
class Tree:
    x: int
    y: int

class Forest:
    def __init__(self, data: List[List[int]]):
        self.forest = nx.MultiDiGraph()
        cardinals = ((0, 1), (1, 0), (0, -1), (-1, 0))
        for y in range(len(data)):
            for x in range(len(data[y])):
                tree = Tree(x=x, y=y)
                tree_height = data[y][x]
                self.forest.add_node(tree, tree_height=tree_height)
                for cardinal in cardinals:
                    try:
                        neighbor_x = tree.x + cardinal[0]
                        neighbor_y = tree.y + cardinal[1]
                        if neighbor_x < 0 or neighbor_y < 0:
                            continue
                        neighbor_height = data[neighbor_y][neighbor_x]
                        neighbor = Tree(x=neighbor_x, y=neighbor_y)
                        if tree_height > neighbor_height:
                            is_taller = True
                        else:
                            is_taller = False

                        self.forest.add_edge(tree, neighbor, is_taller=is_taller)
                    except IndexError:
                        continue


@dataclass
class Tree:
    visible: dict


class Forest
    def __init__(self, data):
        self.forest = {}
        for y in range(len(data)):
            for x in range(len(data[y])):
                self.forest[x, y] = data[y][x]
    print(data)


if __name__ == '__main__':
    with open('day-8-input.txt', 'r') as f:
        data = [[int(height) for height in line.strip()] for line in f.readlines()]

    forest = Forest(data)