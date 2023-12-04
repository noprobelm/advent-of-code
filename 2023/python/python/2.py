import re
import os
from . import PUZZLE_DIR


def main():
    with open(os.path.join(PUZZLE_DIR, "2.txt"), "r") as f:
        data = f.read().strip()
    lines = data.split("\n")

    answer_1 = part_1(lines)
    print(answer_1)

    answer_2 = part_2(lines)
    print(answer_2)


def part_1(lines: list[str]):
    answer = 0
    bag = {"red": 12, "green": 13, "blue": 14}

    for line in lines:
        split = line.split(": ")
        game_id = int("".join([s for s in split[0] if s.isdigit()]))

        data = [d.split() for d in re.split(";|,", split[1])]
        if not any([int(n) > bag[color] for n, color in data]):
            answer += game_id

    return answer


def part_2(lines: list[str]):
    answer = 0
    bag = {"red": 12, "green": 13, "blue": 14}

    for line in lines:
        game = {k: 0 for k in bag.keys()}
        split = line.split(": ")

        data = [d.split() for d in re.split(";|,", split[1])]
        for n, color in data:
            n = int(n)
            if n > game[color]:
                game[color] = n

        answer += game["red"] * game["green"] * game["blue"]

    return answer
