import re
from functools import reduce

puzzle_input = [
    line.strip("\n")
    for line in """123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  """.split("\n")
]

# with open("../../data/6.txt", "r") as f:
#     puzzle_input = [line.strip() for line in f.readlines()]


def part_1():
    answer = 0
    operations = re.findall(r"\S+", puzzle_input[-1])
    sets = [[] for _ in range(len(operations))]
    for line in puzzle_input[:-1]:
        for i, num in enumerate(re.findall(r"\S+", line)):
            sets[i].append(int(num))

    for i, operation in enumerate(operations):
        answer += operate(operation, sets[i])

    return answer


def part_2():
    answer = 0
    operations = re.findall(r"\S+", puzzle_input[-1])
    rows = [list(line) for line in puzzle_input[:-1]]
    transposed = [list(row) for row in zip(*rows)]
    nums = []
    operation = operations[0]
    for i in range(len(transposed)):
        if any(val.isdigit() for val in transposed[i]):
            nums.append(
                int("".join(list(filter(lambda x: x.isdigit(), transposed[i]))))
            )
        # We've encountered a column delimiter. Add the operation to the answer and reset.
        else:
            answer += operate(operation, nums)
            nums = []
            operation = puzzle_input[-1][i + 1]

        # There is no delimiter for the final column. Make sure we catch the end case
        if i == len(transposed) - 1:
            answer += operate(operation, nums)
            nums = []

    return answer


def operate(operation: str, nums: list[int]) -> int:
    match operation:
        case "*":
            return reduce(lambda acc, x: acc * x, nums, 1)
        case "+":
            return sum(nums)
        case _:
            raise ValueError("Expected operator '*' or '+'")


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
