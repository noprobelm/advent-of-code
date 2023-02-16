import re
import math
from rich.console import Console, ConsoleOptions
from rich import print

console = Console()


class Monkey:
    def __init__(
        self, items: list[int], worry_operation, worry_operand, test, test_divisor
    ):
        self.items = items
        self.worry_operation = worry_operation
        self.worry_operand = worry_operand
        self.test = test
        self.test_divisor = test_divisor
        self.num_inspected = 0

    def inspect(self, item):
        worry_operand = self.worry_operand or item
        item = self.worry_operation([item, worry_operand])
        self.num_inspected += 1
        return item

    def get_target(self, item):
        if item % self.test_divisor == 0:
            return self.test[True]
        else:
            return self.test[False]

    def iterate(self, reducer):
        transfer_data = []
        for num, item in enumerate(self.items):
            while item > reducer:
                item %= reducer
            item = self.inspect(item)
            self.items[num] = item
            target = self.get_target(item)
            transfer_data.append([target, item])
        self.items = []
        return transfer_data


class KeepAway:
    def __init__(self, monkeys):
        self.monkeys = [Monkey(*monkey) for monkey in monkeys]
        self.reducer = 1
        for monkey in self.monkeys:
            self.reducer *= monkey.test_divisor

    def play(self, rounds):
        for _ in range(rounds):
            for num, monkey in enumerate(monkeys):
                transfer_data = self.monkeys[num].iterate(self.reducer)
                for data in transfer_data:
                    self.monkeys[data[0]].items.append(data[1])
            print(_)

    @property
    def puzzle_1_answer(self):
        inspect_count_reversed = sorted(
            [monkey.num_inspected for monkey in self.monkeys], reverse=True
        )
        print(inspect_count_reversed)
        return inspect_count_reversed[0] * inspect_count_reversed[1]

    @property
    def puzzle_2_answer(self):
        inspect_count_reversed = sorted(
            [monkey.num_inspected for monkey in self.monkeys], reverse=True
        )
        print(inspect_count_reversed)
        return inspect_count_reversed
        return inspect_count_reversed[0] * inspect_count_reversed[1]


def parse_puzzle_input():
    monkeys = []
    with open("day-11-input.txt", "r") as f:
        data = f.read()
        data = [re.split("\n", item)[1:] for item in re.split("\n\n", data)]

    for idx, monkey in enumerate(data):
        items = [int(match) for match in re.findall(r"\d+", monkey[0])]
        worry_operation = math.prod if "*" in monkey[1] else sum
        if re.search("\d+", monkey[1]):
            worry_operand = int(re.search("\d+", monkey[1]).group())
        else:
            worry_operand = None
        test_divisor = int(re.search("\d+", monkey[2]).group())
        test = {
            True: int(re.search("\d+", monkey[3]).group()),
            False: int(re.search("\d+", monkey[4]).group()),
        }
        monkeys.append([items, worry_operation, worry_operand, test, test_divisor])
    return monkeys


if __name__ == "__main__":
    monkeys = parse_puzzle_input()
    keepaway = KeepAway(monkeys)
    #    keepaway.play(20)
    keepaway.play(10000)
    print(keepaway.puzzle_2_answer)
