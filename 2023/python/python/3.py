import re
import os
from . import PUZZLE_DIR


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
    for row, line in enumerate(lines):
        nums = re.finditer(r"\d+", "".join(line))
        for k in nums:
            n = k.group()
            span = range(*k.span())
            part = Part(n, row, span)
            neighbors = part.find_neighbors(matrix)
            filtered = list(filter(lambda s: s != "." or s.isdigit(), neighbors))
            if len(filtered) > 0:
                answer += int(n)

    return answer


def part_2(lines: list[str]):
    answer = 0
    matrix = [list(line) for line in lines]
    for row, line in enumerate(lines):
        gears = re.finditer(r"\*", "".join(line))
        for k in gears:
            val = k.group()
            span = range(*k.span())
            part = Part(val, row, span)
            adjacent_part_numbers = part.find_adjacent_part_numbers(matrix)
            if len(adjacent_part_numbers) == 2:
                answer += adjacent_part_numbers[0] * adjacent_part_numbers[1]

    return answer


class Part:
    def __init__(self, val: str, row: int, span: range):
        self.val = val
        self.row = row
        self.span = span

    def find_neighbors(self, table_data: list[list[str]]):
        neighbors = []
        if self.row == 0:
            y1 = self.row
        else:
            y1 = self.row - 1

        if self.row == len(table_data) - 1:
            y2 = self.row
        else:
            y2 = self.row + 1

        if self.span[0] == 0:
            x1 = 0
        else:
            x1 = self.span[0] - 1

        if self.span[-1] == len(table_data[self.row]) - 1:
            x2 = self.span[-1]
        else:
            x2 = self.span[-1] + 1

        for row in range(y1, y2 + 1):
            if row == self.row:
                if x1 > 0:
                    neighbors.append(table_data[row][x1])
                if x2 < len(table_data[self.row]) - 1:
                    neighbors.append(table_data[row][x2])
            else:
                for i in range(x1, x2 + 1):
                    neighbors.append(table_data[row][i])
        return neighbors

    def find_adjacent_part_numbers(self, table_data: list[list[str]]):
        part_numbers = []
        if self.row == 0:
            y1 = self.row
        else:
            y1 = self.row - 1

        if self.row == len(table_data) - 1:
            y2 = self.row
        else:
            y2 = self.row + 1

        if self.span[0] == 0:
            x1 = 0
        else:
            x1 = self.span[0] - 1

        if self.span[-1] == len(table_data[self.row]) - 1:
            x2 = self.span[-1]
        else:
            x2 = self.span[-1] + 1

        adjacent = []
        for row in range(y1, y2 + 1):
            if row == self.row:
                i = x1
                while i >= 0 and table_data[row][i].isdigit():
                    adjacent.append(table_data[row][i])
                    i -= 1
                if len(adjacent) > 0:
                    n = int("".join(list(reversed(adjacent))))
                    part_numbers.append(n)
                    adjacent = []
                i = x2
                while i < len(table_data[row]) and table_data[row][i].isdigit():
                    adjacent.append(table_data[row][i])
                    i += 1
                if len(adjacent) > 0:
                    n = int("".join(adjacent))
                    part_numbers.append(n)
                    adjacent = []
            else:
                start = x1
                while table_data[row][start].isdigit() and start > 0:
                    start -= 1
                end = x2
                while table_data[row][end].isdigit() and end < len(table_data) - 1:
                    end += 1

                for i in table_data[row][start : end + 1]:
                    if i.isdigit():
                        adjacent.append(i)
                    elif len(adjacent) > 0:
                        n = int("".join(list(adjacent)))
                        part_numbers.append(n)
                        adjacent = []

        return part_numbers
