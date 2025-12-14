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
        grid.append([])
        for val in row:
            grid[y].append(val)
    grid = Grid(grid)

    for position, val in grid.enumerate():
        if (
            val == "@"
            and len(
                [
                    neighbor
                    for neighbor in MooreNeighborhood(position).get_neighbors()
                    if grid.get(neighbor) == "@"
                ]
            )
            < 4
        ):
            answer += 1

    return answer


def part_2():
    grid = []
    num_rolls = 0
    for y, row in enumerate(puzzle_input):
        grid.append([])
        for val in row:
            grid[y].append(val)
            if val == "@":
                num_rolls += 1
    grid = Grid(grid)
    while True:
        num_removed = 0
        for position, val in grid.enumerate():
            if (
                val == "@"
                and len(
                    [
                        neighbor
                        for neighbor in MooreNeighborhood(position).get_neighbors()
                        if grid.get(neighbor) == "@"
                    ]
                )
                < 4
            ):
                grid.set(position, ".")
                num_removed += 1

        if num_removed == 0:
            break

    return num_rolls - len([val for val in grid if val == "@"])


@dataclass
class Grid:
    grid: list[list[str]]

    def __post_init__(self):
        self.shape = (len(self.grid), len(self.grid[0]))

    def __contains__(self, position: Position):
        return (
            position.y < self.shape[0]
            and position.y >= 0
            and position.x < self.shape[1]
            and position.x >= 0
        )

    def get(self, position: Position):
        if position in self:
            return self.grid[position.y][position.x]
        else:
            return None

    def set(self, position: Position, val: str):
        self.grid[position.y][position.x] = val

    def enumerate(self):
        for y, row in enumerate(self.grid):
            for x, value in enumerate(row):
                yield Position(x, y), value

    def __iter__(self):
        for row in self.grid:
            for value in row:
                yield value


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
