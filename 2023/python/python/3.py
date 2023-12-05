import re
import os
from collections import namedtuple
from . import PUZZLE_DIR


Point = namedtuple("Point", ["x", "y"])


def main():
    with open(os.path.join(PUZZLE_DIR, "3.txt"), "r") as f:
        data = f.read().strip()
    lines = data.split("\n")

    answer_1 = part_1(lines)
    print(f"Part 1: {answer_1}")

    answer_2 = part_2(lines)
    print(f"Part 2: {answer_2}")


def part_1(lines: list[str]):
    answer = 0
    matrix = [list(line) for line in lines]
    y_max = len(matrix) - 1
    for row, line in enumerate(lines):
        x_max = len(line) - 1
        nums = re.finditer(r"\d+", "".join(line))
        for i in nums:
            val = i.group()
            span = range(*i.span())
            neighbor_positions = neighbors_xy(row, span, x_max, y_max)
            neighbors = [matrix[p.y][p.x] for p in neighbor_positions]
            if len(list(filter(lambda s: s != "." or s.isdigit(), neighbors))) > 0:
                answer += int(val)

    return answer


def part_2(lines: list[str]):
    answer = 0
    matrix = [list(line) for line in lines]
    y_max = len(matrix)
    for row, line in enumerate(lines):
        x_max = len(line)
        gears = re.finditer(r"\*", "".join(line))
        for i in gears:
            nums = []
            span = range(*i.span())
            neighbor_positions = neighbors_xy(row, span, x_max, y_max)
            above = list(filter(lambda p: p.y == row - 1, neighbor_positions))
            current = list(filter(lambda p: p.y == row, neighbor_positions))
            below = list(filter(lambda p: p.y == row + 1, neighbor_positions))

            for r in [above, current, below]:
                num = []
                y = r[0].y
                neighbor_x_min = min([p.x for p in r])
                neighbor_x_max = max([p.x for p in r])
                while neighbor_x_min > 0 and matrix[y][neighbor_x_min].isdigit():
                    neighbor_x_min -= 1
                while neighbor_x_max < x_max and matrix[y][neighbor_x_max].isdigit():
                    neighbor_x_max += 1

                for x in range(neighbor_x_min, neighbor_x_max):
                    c = matrix[y][x]
                    if c.isdigit():
                        num.append(c)
                    elif len(num) > 0:
                        nums.append(int("".join(num)))
                        num = []

                if len(num) > 0:
                    nums.append(int("".join(num)))
                    num = []

            if len(nums) == 2:
                answer += nums[0] * nums[1]

    return answer


def neighbors_xy(row: int, span: range, x_max: int, y_max: int) -> list[Point]:
    neighbors = []
    x1 = 0 if span[0] == 0 else span[0] - 1
    x2 = span[-1] if span[-1] == x_max else span[-1] + 1

    y1 = 0 if row == 0 else row - 1
    y2 = row if row == y_max else row + 1

    for i in range(y1, y2 + 1):
        if i != row:
            for k in range(x1, x2 + 1):
                neighbors.append(Point(k, i))

        else:
            if x1 < span[0]:
                neighbors.append(Point(x1, i))
            if x2 > span[-1]:
                neighbors.append(Point(x2, i))

    return neighbors
