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
            packets.append(eval(f"list({line})"))

    return packets


def get_pairs(packets):
    pairs = []
    for num, packet in enumerate(packets):
        num += 1
        if num % 2 == 1:
            pairs.append([packet, packets[num]])

    return pairs


def compare_ints(left, right):
    if left < right:
        return True
    elif left > right:
        return False
    else:
        return None


def is_ordered(left, right):
    for i in range(max([len(left), len(right)])):
        if i + 1 > len(left):
            return True
        elif i + 1 > len(right):
            return False
        elif isinstance(left[i], list) and isinstance(right[i], list):
            ordered = is_ordered(left[i], right[i])
            if ordered is not None:
                return ordered
        elif isinstance(left[i], int) and isinstance(right[i], int):
            ordered = compare_ints(left[i], right[i])
            if ordered is not None:
                return ordered
        elif isinstance(left[i], int) and isinstance(right[i], list):
            ordered = is_ordered([left[i]], right[i])
            if ordered is not None:
                return ordered

        elif isinstance(right[i], int) and isinstance(left[i], list):
            ordered = is_ordered(left[i], [right[i]])
            if ordered is not None:
                return ordered


def bubble_sort(packets):
    for i in range(len(packets)):
        already_sorted = True
        for j in range(len(packets) - i - 1):
            if is_ordered(packets[j], packets[j + 1]) is False:
                packets[j], packets[j + 1] = packets[j + 1], packets[j]
                already_sorted = False

        if already_sorted:
            break
    return packets


if __name__ == "__main__":
    with open("day-13-input.txt", "r") as f:
        input_str = f.read()
    packets = get_packets(input_str)
    pairs = get_pairs(packets)
    ordered_idx = []
    for num, pair in enumerate(pairs):
        ordered_idx.append(is_ordered(pair[0], pair[1]))
    print(sum([i + 1 for i in range(len(ordered_idx)) if ordered_idx[i] is True]))
    packets.append([[2]])
    packets.append([[6]])

    packets = bubble_sort(packets)
    divider_idx = [
        packets.index(packet) + 1
        for packet in packets
        if packet == [[6]] or packet == [[2]]
    ]
    print(divider_idx[0] * divider_idx[1])
