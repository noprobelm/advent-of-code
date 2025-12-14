from rich import print

puzzle_input = [
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
]

with open("../../data/3.txt", "r") as f:
    puzzle_input = [line.strip() for line in f.readlines()]


def part_1():
    joltages = []
    for battery in puzzle_input:
        battery = list(battery.strip())
        largest = max(battery[: len(battery) - 1])
        next_largest = max(battery[battery.index(largest) + 1 :])
        joltages.append(int("".join([largest, next_largest])))

    return sum(joltages)


def part_2():
    joltages = []
    for battery in puzzle_input:
        joltage = []
        i = 0
        skips_remaining = len(battery) - 12
        while i < len(battery) and len(joltage) < 12:
            local_max_idx = i
            for k in range(local_max_idx, local_max_idx + skips_remaining + 1):
                if k < len(battery) and battery[k] > battery[local_max_idx]:
                    local_max_idx = k
            skips_remaining -= local_max_idx - i
            joltage.append(battery[local_max_idx])
            i = local_max_idx + 1
        joltages.append(int("".join(joltage)))

    return sum(joltages)


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
