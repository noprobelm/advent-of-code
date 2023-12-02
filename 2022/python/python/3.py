from rich.console import Console

console = Console()
from string import ascii_letters


def main():
    with open("../../data/3.txt", "r") as f:
        rucksacks = [Rucksack(line.replace("\n", "")) for line in f.readlines()]

    start = 0
    end = 3
    groups = []
    while end <= len(rucksacks):
        groups.append(GroupInventory(rucksacks[start:end]))
        start += 3
        end += 3

    part_1 = sum([rucksack.priority for rucksack in rucksacks])
    part_2 = sum([group.priority for group in groups])

    print(f"Part 1: {part_1}")
    print(f"Part 2: {part_2}")


class Rucksack:
    priorities = dict(zip(ascii_letters, range(1, 53)))

    def __init__(self, contents: str):
        self.contents = contents
        self.compartments = [
            contents[len(contents) // 2 :],
            contents[: len(contents) // 2],
        ]
        self.common = self.find_common()

    def find_common(self):
        for item in self.compartments[0]:
            if item in self.compartments[1]:
                return item

    @property
    def priority(self):
        return self.priorities[self.common]


class GroupInventory:
    priorities = dict(zip(ascii_letters, range(1, 53)))

    def __init__(self, rucksacks: list[Rucksack]):
        self.rucksacks = rucksacks
        self.common = self.find_common()

    def find_common(self):
        for item in self.rucksacks[0].contents:
            if (
                item in self.rucksacks[1].contents
                and item in self.rucksacks[2].contents
            ):
                return item

    @property
    def priority(self):
        return self.priorities[self.common]


if __name__ == "__main__":
    main()
