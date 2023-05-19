from rich.console import Console
from collections import namedtuple
from functools import reduce
import operator

console = Console()
Tree = namedtuple("Tree", "x y")


class Forest(dict):
    def __init__(self, data: list[list]) -> None:
        """
        Accepts a 2d list with axis of equal length; each field is interpreted as the height of a tree.

        The result is a subclassed dict of these characteristics:
        dict[namedtuple[x, y], height[int], visible[bool], visibliity_score[int]]
        """
        trees = {}
        for y in range(len(data)):
            for x in range(len(data[y])):
                trees[Tree(x, y)] = {"height": data[y][x]}

        self.x_min = 0
        self.y_min = 0
        self.x_max = len([tree for tree in data[0]]) - 1
        self.y_max = len(data) - 1

        super().__init__(trees)

        for tree in self:
            self.get_visibility(tree)

    @property
    def num_visible(self) -> int:
        return len([tree for tree in self if self[tree]["visible"]])

    @property
    def highest_score(self) -> tuple:
        highest_score = max([forest[tree]["visibility_score"] for tree in forest])
        highest_scoring_tree = list(
            filter(
                lambda tree: forest[tree]["visibility_score"] == highest_score, forest
            )
        )

        return highest_scoring_tree, highest_score

    def get_visibility(self, tree: Tree) -> None:
        """
        Determine the visibility and visibility scores of a tree. The algorithm achieves this by incrementing or
        decrementing x and y from the target tree's position, comparing height values, and incrementing the score as
        necessary until all relevant neighbors have been explored.

        Visible from the outside:
        As we explore a tree's neighbors to calculate visibility score, we simulataneously determine visibility from the
        outside:
            - If a tree's x or y position is found to be xmin, xmax, ymin, or ymax, it is deemed visible from the outside.
            - If not, visibility is deemed True if a relevant neighbor's xpos or ypos are found to be equal to these
              values.
            - If neither of these cases are encountered, the tree is not visible from the outside from the cardinal
              direction.
        """
        scores = {"north": 0, "east": 0, "south": 0, "west": 0}

        if tree.y > self.y_min:
            for y in range(1, tree.y + 1):
                neighbor = Tree(tree.x, tree.y - y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["north"] += 1
                    if neighbor.y == self.y_min:
                        visible_north = True
                    else:
                        visible_north = False
                else:
                    scores["north"] += 1
                    visible_north = False
                    break
        else:
            visible_north = True

        if tree.y < self.y_max:
            for y in range(tree.y, self.y_max):
                neighbor = Tree(tree.x, y + 1)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["south"] += 1
                    if neighbor.y == self.y_max:
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
            for x in range(tree.x, self.x_max):
                neighbor = Tree(x + 1, tree.y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["east"] += 1
                    if neighbor.x == self.x_max:
                        visible_east = True
                    else:
                        visible_east = False

                else:
                    scores["east"] += 1
                    visible_east = False
                    break

        else:
            visible_east = True

        if tree.x > self.x_min:
            for x in range(1, tree.x + 1):
                neighbor = Tree(tree.x - x, tree.y)
                if self[neighbor]["height"] < self[tree]["height"]:
                    scores["west"] += 1
                    if neighbor.x == self.x_min:
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

        scores = [scores[cardinal] for cardinal in scores if scores[cardinal] > 0]
        self[tree]["visibility_score"] = reduce(operator.mul, scores)


if __name__ == "__main__":
    with open("day-8-input.txt", "r") as f:
        data = [[int(height) for height in line.strip()] for line in f.readlines()]

    forest = Forest(data)

    console.print(
        f"The number of visible trees from each perspective along the perimeter is {forest.num_visible}"
    )

    highest_scoring = forest.highest_score
    console.print(
        f"The tree with the highest visibility score is {highest_scoring[0]}, with a score of {highest_scoring[1]}"
    )
