import regex as re
from . import PUZZLE_DIR
import os
from aoc import Puzzle

print(help("modules"))


def part_2():
    with open(os.path.join(PUZZLE_DIR, "1.txt"), "r") as f:
        data = f.read().strip()

    lines = data.split("\n")
    mapper = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    pattern = [f"({k})" for k in mapper.keys()]
    pattern.extend([str(v) for v in mapper.values()])
    pattern = re.compile("|".join(pattern))

    for line in lines:
        matches = re.findall(pattern, line, overlapped=True)


def main():
    part_2()
