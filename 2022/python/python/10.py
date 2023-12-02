from rich import print
from rich.console import Console, ConsoleOptions

console = Console()


def main():
    with open("../../data/10.txt", "r") as f:
        instructions = [line.strip("\n").split(" ") for line in f.readlines()]
        for idx, instruction in enumerate(instructions):
            if instruction[0] == "noop":
                instructions[idx] = noop()
            elif instruction[0] == "addx":
                instructions[idx] = addx(int(instruction[1]))

    cpu = CPU()
    for instruction in instructions:
        cpu.eval(instruction)
    answer = 0
    cycles = [i + 1 for i in range(220)][19::40]
    for cycle in cycles:
        answer += cpu.history[cycle] * cycle
    print(f"Part 1: {answer}")
    print("Part 2:")
    console.print(cpu.crt)


class Instruction:
    def __init__(self, x, cycles):
        self.x = x
        self.cycles = cycles

    def __len__(self):
        return self.cycles


class noop(Instruction):
    def __init__(self):
        super().__init__(0, 1)


class addx(Instruction):
    def __init__(self, x):
        super().__init__(x, 2)


class CRT:
    def __init__(self, resolution_x, resolution_y):
        self.resolution = (resolution_x, resolution_y)
        self.pixels = []
        self.drawn = []

    def draw(self, cycle, sprite):
        cycle = (cycle - 1) % self.resolution[0]
        if cycle in sprite:
            self.pixels.append("#")
        else:
            self.pixels.append(".")

    def __getitem__(self, index):
        return self.pixels[index]

    def __setitem__(self, index, val):
        self.pixels[index] = val

    def __rich_console__(self, console, options):
        start = 0
        end = self.resolution[0]
        for row in range(self.resolution[1]):
            yield "".join(self.pixels[start:end])
            start += self.resolution[0]
            end += self.resolution[0]


class CPU:
    cycle: int = 1
    x: int = 1

    def __init__(self):
        self.crt = CRT(40, 6)
        self.history = {}

    @property
    def sprite(self):
        return [self.x - 1, self.x, self.x + 1]

    def eval(self, instruction: Instruction):
        for cycle in range(len(instruction)):
            self.history[self.cycle] = self.x
            self.crt.draw(self.cycle, self.sprite)
            self.cycle += 1
        self.x += instruction.x
        self.history[self.cycle] = self.x


if __name__ == "__main__":
    main()
