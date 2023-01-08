import numpy as np
from rich.console import Console
from dataclasses import dataclass
console = Console()


@dataclass
class Tree:
    x: int
    y: int
    height: int = 0
    is_visible: bool = None
    visibility_score: int = 0

    def __hash__(self):
        return hash((self.x, self.y))

    def __eq__(self, other):
        if other.x == self.x and other.y == self.y:
            return True


class Forest:
    def __init__(self, data):
        self.input_data = data
        self.forest = {}
        for y in range(len(data)):
            for x in range(len(data[y])):
                self.forest[Tree(x, y, height=0)] = {}

    @property
    def num_visible(self):
        return sum([1 for tree in self.forest if tree.is_visible])

    @staticmethod
    def get_row_visibility(row):
        trees = [True]
        tallest = row[0]
        for tree in row[1:]:
            if tree <= tallest:
                visible = False
            else:
                visible = True
                tallest = tree
            trees.append(visible)
        return trees

    def run(self):
        directions = {'west': 1, 'east': -1, 'north': 1, 'south': -1}
        data = np.array(self.input_data)
        for y in range(len(data)):
            for direction in ['west', 'east']:
                visibility = self.get_row_visibility(data[y][::directions[direction]])
                visibility = visibility[::directions[direction]]
                for x in range(len(data[y])):
                    self.forest[Tree(x, y)][direction] = visibility[x]
        data = np.transpose(data)
        for x in range(len(data)):
            for direction in ['north', 'south']:
                visibility = self.get_row_visibility(data[x][::directions[direction]])
                visibility = visibility[::directions[direction]]
                for y in range(len(data[x])):
                    self.forest[Tree(x, y)][direction] = visibility[y]

        for tree in self.forest:
            for direction in self.forest[tree].copy():
                if self.forest[tree][direction] is True:
                    tree.is_visible = True
                    break
                else:
                    tree.is_visible = False


if __name__ == '__main__':
    with open('day-8-input.txt', 'r') as f:
        data = [[int(height) for height in line.strip()] for line in f.readlines()]

    forest = Forest(data)
    forest.run()
    arr = np.array(forest.num_visible)
    visible = forest.num_visible
    print(visible)