puzzle_input = [
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
]

# with open("../../data/3.txt", "r") as f:
#     puzzle_input = f.readlines()


def part_1():
    jolts = []
    for battery in puzzle_input:
        battery = list(battery.strip())
        largest = max(battery[: len(battery) - 1])
        next_largest = max(battery[battery.index(largest) + 1 :])
        jolts.append(int("".join([largest, next_largest])))

    return sum(jolts)


def part_2():
    pass


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    # answer = part_2()
    # print(answer)
