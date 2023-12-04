import regex as re
import os
from . import PUZZLE_DIR


def main():
    with open(os.path.join(PUZZLE_DIR, "1.txt"), "r") as f:
        data = f.read().strip()
    lines = data.split("\n")

    answer_1 = part_1(lines)
    print(answer_1)

    answer_2 = part_2(lines)
    print(answer_2)


def part_1(lines: list[str]):
    answer = 0

    for line in lines:
        nums = []
        for s in line:
            if s.isdigit():
                nums.append(s)

        num = int(f"{nums[0]}{nums[-1]}")
        answer += num

    return answer


def part_2(lines: list[str]):
    answer = 0

    mapper = {
        "one": "1",
        "two": "2",
        "three": "3",
        "four": "4",
        "five": "5",
        "six": "6",
        "seven": "7",
        "eight": "8",
        "nine": "9",
    }
    pattern = "|".join(mapper.keys())
    pattern = rf"{pattern}|\d"

    for line in lines:
        matches = re.findall(pattern, line, overlapped=True)

        if mapper.get(matches[0]):
            first = mapper[matches[0]]
        else:
            first = matches[0]

        if mapper.get(matches[-1]):
            last = mapper[matches[-1]]
        else:
            last = matches[-1]

        num = int(f"{first}{last}")
        answer += num

    return answer
