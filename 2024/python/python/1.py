import os
from . import PuzzleInput

def main() -> None:
    puzzle_input = PuzzleInput("1.txt")
    lines = puzzle_input.lines()

    left = []
    right = []
    for i, line in enumerate(lines):
        left_n, right_n = line.split("   ")
        left.append(int(left_n))
        right.append(int(right_n))

    distance = part_1(left, right)
    print(f"Part 1: {distance}")

    count = part_2(left, right)
    print(f"Part 2: {count}")

def part_1(left: list[int], right: list[int]) -> int:
    distance = 0
    left = list(sorted(left))
    right = list(sorted(right))

    for (left_n, right_n) in zip(left, right):
        distance += abs(left_n - right_n)

    return distance

def part_2(left: list[int], right: list[int]) -> int:
    count = 0
    for left_n in left:
        matched = list(filter(lambda right_n: right_n == left_n, right))
        count += left_n * len(matched)

    return count

if __name__ == "__main__":
    main()
