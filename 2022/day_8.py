from rich.console import Console
from collections import namedtuple
from functools import reduce
import operator

console = Console()
Tree = namedtuple("Tree", "x y")


class Forest(dict):
    def __init__(self, data: list[list]):
        trees = {}
        for y in range(len(data)):
            for x in range(len(data[y])):
                trees[Tree(x, y)] = {"height": data[y][x]}
        super().__init__(trees)

    def get_neighbors(self, tree):
        scores = {"north": 0, "east": 0, "south": 0, "west": 0}

        if tree.y > 0:
            for y in range(1, tree.y + 1):
                neighbor = Tree(tree.x, tree.y - y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["north"] += 1
                    if neighbor.y == 0:
                        visible_north = True
                    else:
                        visible_north = False
                else:
                    scores["north"] += 1
                    visible_north = False
                    break
        else:
            visible_north = True

        if tree.y < 98:
            for y in range(tree.y, 98):
                neighbor = Tree(tree.x, y + 1)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["south"] += 1
                    if neighbor.y == 98:
                        visible_south = True
                    else:
                        visible_south = False
                else:
                    scores["south"] += 1
                    visible_south = False
                    break

        else:
            visible_south = True

        if tree.x < 98:
            for x in range(tree.x, 98):
                neighbor = Tree(x + 1, tree.y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["east"] += 1
                    if neighbor.x == 98:
                        visible_east = True
                    else:
                        visible_east = False

                else:
                    scores["east"] += 1
                    visible_east = False
                    break

        else:
            visible_east = True

        if tree.x > 0:
            for x in range(1, tree.x + 1):
                neighbor = Tree(tree.x - x, tree.y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["west"] += 1
                    if neighbor.x == 0:
                        visible_west = True
                    else:
                        visible_west = False
                else:
                    scores["west"] += 1
                    visible_west = False
                    break

        else:
            visible_west = True

        self[tree]["visible"] = any(
            [visible_north, visible_east, visible_west, visible_south]
        )

        scores = [scores[score] for score in scores if scores[score] > 0]
        self[tree]["visibility_score"] = reduce(operator.mul, scores)


if __name__ == "__main__":
    with open("day-8-input.txt", "r") as f:
        data = [[int(height) for height in line.strip()] for line in f.readlines()]

    forest = Forest(data)
    for tree in forest:
        forest.get_neighbors(tree)

    num_visible = [tree for tree in forest if forest[tree]["visible"]]
    highest_score = max([forest[tree]["visibility_score"] for tree in forest])
    print(highest_score)
