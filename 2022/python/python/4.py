import re


def main():
    with open("../../data/4.txt", "r") as f:
        sets = []
        for line in f.readlines():
            line = line.strip()
            line = re.split(",", line)
            line[0] = [int(num) for num in re.findall(r"(\d+)", line[0])]
            first = set(list(range(line[0][0], line[0][1] + 1)))
            line[1] = [int(num) for num in re.findall(r"(\d+)", line[1])]
            second = set(list(range(line[1][0], line[1][1] + 1)))
            sets.append([first, second])

    part_1 = 0
    part_2 = 0
    for pair in sets:
        if pair[0].issubset(pair[1]) or pair[1].issubset(pair[0]):
            part_1 += 1
        if pair[0].intersection(pair[1]):
            part_2 += 1

    print(f"Part 1: {part_1}")
    print(f"Part 2: {part_2}")


if __name__ == "__main__":
    with open("day-4-input.txt", "r") as f:
        sets = []
        for line in f.readlines():
            line = line.strip()
            line = re.split(",", line)
            line[0] = [int(num) for num in re.findall(r"(\d+)", line[0])]
            first = set(list(range(line[0][0], line[0][1] + 1)))
            line[1] = [int(num) for num in re.findall(r"(\d+)", line[1])]
            second = set(list(range(line[1][0], line[1][1] + 1)))
            sets.append([first, second])

    num_subsets = 0
    intersections = 0
    for pair in sets:
        if pair[0].issubset(pair[1]) or pair[1].issubset(pair[0]):
            num_subsets += 1
        if pair[0].intersection(pair[1]):
            intersections += 1

    print(
        f"The number of elf's tasks who fully overlap their partner's tasks is {num_subsets}"
    )
    print(
        f"The number of elf's tasks who have any degree of overlap with their partner's tasks is {intersections}"
    )
