import re
import math
from rich.console import Console, ConsoleOptions
from rich import print

console = Console()


class Monkey:
    def __init__(
        self, items: list[int], worry_operation, worry_operand, test_dict, test_divisor
    ):
        self.items = items
        self.worry_operation = worry_operation
        self.worry_operand = worry_operand
        self.test_dict = test_dict
        self.test_divisor = test_divisor
        self.num_inspected = 0

    def inspect(self, cycle_length, part=1):
        transfer_data = {monkey: [] for monkey in self.test_dict.values()}
        for item in self.items:
            worry_operand = self.worry_operand or item
            item = self.worry_operation([item, worry_operand])
            if part == 1:
                item /= 3
            else:
                while item > cycle_length:
                    item %= cycle_length

            self.num_inspected += 1

            if item % self.test_divisor == 0:
                transfer_data[self.test_dict[True]].append(item)
            else:
                transfer_data[self.test_dict[False]].append(item)

        return transfer_data


class KeepAway:
    def __init__(self, monkeys):
        self.monkeys = [Monkey(*monkey) for monkey in monkeys]
        self.cycle_length = 1
        for monkey in self.monkeys:
            self.cycle_length *= monkey.test_divisor

    def play(self, rounds, part):
        for _ in range(rounds):
            for num, origin_monkey in enumerate(monkeys):
                transfer_data = self.monkeys[num].inspect(self.cycle_length)
                for target_monkey in transfer_data:
                    for item in transfer_data[target_monkey]:
                        self.monkeys[target_monkey].items.append(item)
                self.monkeys[num].items = []

    @property
    def puzzle_answer(self):
        inspect_count_reversed = sorted(
            [monkey.num_inspected for monkey in self.monkeys], reverse=True
        )
        return inspect_count_reversed[0] * inspect_count_reversed[1]


def parse_puzzle_input():
    monkeys = []
    with open("day-11-input.txt", "r") as f:
        data = f.read()
        data = [re.split("\n", item)[1:] for item in re.split("\n\n", data)]

    for idx, monkey in enumerate(data):
        items = [int(match) for match in re.findall(r"\d+", monkey[0])]
        worry_operation = math.prod if "*" in monkey[1] else sum
        if re.search(r"\d+", monkey[1]):
            worry_operand = int(re.search(r"\d+", monkey[1]).group())
        else:
            worry_operand = None
        test_divisor = int(re.search(r"\d+", monkey[2]).group())
        test_dict = {
            True: int(re.search(r"\d+", monkey[3]).group()),
            False: int(re.search(r"\d+", monkey[4]).group()),
        }
        monkeys.append([items, worry_operation, worry_operand, test_dict, test_divisor])
    return monkeys


if __name__ == "__main__":
    monkeys = parse_puzzle_input()
    keepaway = KeepAway(monkeys)
    keepaway.play(20, part=1)

    # keepaway = KeepAway(monkeys)
    # keepaway.play(10000, part=2
    print(keepaway.puzzle_answer)
