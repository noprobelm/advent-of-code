import re

SAMPLE = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""


def get_packets(input_str: str) -> list:
    puzzle_parsed = []
    packets = []
    input_str = input_str.split("\n")
    for line in input_str:
        if line.startswith("["):
            puzzle_parsed.append(eval(f"list({line})"))

    for num, packet in enumerate(puzzle_parsed):
        num += 1
        if num % 2 == 1:
            packets.append([packet, puzzle_parsed[num]])

    return packets


def compare_ints(left, right):
    if left < right:
        return True
    elif left > right:
        return False
    else:
        return None


def compare_lists(left, right):
    for i in range(max([len(left), len(right)])):
        if i + 1 > len(left):
            return True
        elif i + 1 > len(right):
            return False
        elif isinstance(left[i], list) and isinstance(right[i], list):
            ordered = compare_lists(left[i], right[i])
            if ordered is not None:
                return ordered
        elif isinstance(left[i], int) and isinstance(right[i], int):
            ordered = compare_ints(left[i], right[i])
            if ordered is not None:
                return ordered
        elif isinstance(left[i], int) and isinstance(right[i], list):
            ordered = compare_lists([left[i]], right[i])
            if ordered is not None:
                return ordered

        elif isinstance(right[i], int) and isinstance(left[i], list):
            ordered = compare_lists(left[i], [right[i]])
            if ordered is not None:
                return ordered


def compare_different(left, right):
    pass


if __name__ == "__main__":
    with open("day-13-input.txt", "r") as f:
        input_str = f.read()
    packets = get_packets(input_str)
    ordered_idx = []
    for num, pair in enumerate(packets):
        ordered = []
        left = pair[0]
        right = pair[1]
        ordered_idx.append(compare_lists(left, right))
    print(sum([i + 1 for i in range(len(ordered_idx)) if ordered_idx[i] is True]))
