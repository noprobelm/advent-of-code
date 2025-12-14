from __future__ import annotations
from dataclasses import dataclass

puzzle_input = [
    "..@@.@@@@.",
    "@@@.@.@.@@",
    "@@@@@.@.@@",
    "@.@@@@..@.",
    "@@.@@@@.@@",
    ".@@@@@@@.@",
    ".@.@.@.@@@",
    "@.@@@.@@@@",
    ".@@@@@@@@.",
    "@.@.@@@.@.",
]

with open("../../data/4.txt", "r") as f:
    puzzle_input = [line.strip() for line in f.readlines()]


def part_1():
    answer = 0
    grid = []
    for y, row in enumerate(puzzle_input):
        for x, val in enumerate(row):
            if val == "@":
                grid.append((Position(x, y)))
    for position in grid:
        adjacent_rolls = len(
            [
                neighbor
                for neighbor in MooreNeighborhood(position).get_neighbors()
                if neighbor in grid
            ]
        )
        if adjacent_rolls < 4:
            answer += 1

    return answer


def part_2():
    grid = []
    for y, row in enumerate(puzzle_input):
        for x, val in enumerate(row):
            if val == "@":
                grid.append((Position(x, y)))
    num_rolls = len(grid)
    while True:
        removed = []
        for position in grid:
            adjacent_rolls = len(
                [
                    neighbor
                    for neighbor in MooreNeighborhood(position).get_neighbors()
                    if neighbor in grid
                ]
            )
            if adjacent_rolls < 4:
                removed.append(position)
        if len(removed) == 0:
            break
        grid = [position for position in grid if position not in removed]
        print(len(removed))
        removed = []

    return num_rolls - len(grid)


@dataclass(frozen=True, slots=True)
class Position:
    x: int
    y: int

    def __add__(self, other: Position) -> Position:
        return Position(self.x + other.x, self.y + other.y)

    def __sub__(self, other: Position) -> Position:
        return Position(self.x - other.x, self.y - other.y)


class MooreNeighborhood:
    neighbors: tuple[Position, ...] = (
        Position(-1, -1),
        Position(0, -1),
        Position(1, -1),
        Position(1, 0),
        Position(1, 1),
        Position(0, 1),
        Position(-1, 1),
        Position(-1, 0),
    )

    def __init__(self, position: Position) -> None:
        """Initialize an instance of the MooreCell class."""
        self.position = position

    def get_neighbors(self) -> list[Position]:
        return [pos + self.position for pos in self.neighbors]


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
