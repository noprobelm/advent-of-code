puzzle_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

with open("../../data/2.txt", "r") as f:
    puzzle_input = f.read()


def part_1():
    invalid_ids = []
    for id_range in puzzle_input.split(","):
        start, end = id_range.split("-")
        for id in range(int(start), int(end) + 1):
            id_str = str(id)
            if (
                len(id_str) % 2 == 0
                and id_str[: len(id_str) // 2] == id_str[len(id_str) // 2 :]
            ):
                invalid_ids.append(id)

    return sum(invalid_ids)


def part_2():
    invalid_ids = []
    for id_range in puzzle_input.split(","):
        start, end = id_range.split("-")
        for id in range(int(start), int(end) + 1):
            id_str = str(id)
            for i in range(len(id_str)):
                sequence = id_str[: i + 1]
                split = id_str.split(sequence)
                if len(split) > 2 and all(["" == s for s in id_str.split(sequence)]):
                    invalid_ids.append(id)
                    break

    return sum(invalid_ids)


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
