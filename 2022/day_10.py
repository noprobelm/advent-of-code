from rich import print
from rich.console import Console, ConsoleOptions


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
    resolution = (40, 6)

    def __init__(self):
        self.pixels = ["" for _ in range(self.resolution[0] * self.resolution[1])]
        self.drawn = []

    def draw(self, cycle, sprite):
        cycle -= 1
        if cycle % self.resolution[0] == 0 and cycle > 0:
            print(True)
            for pixel in self.pixels[:40]:
                self.drawn.append(pixel)
            self.pixels = self.pixels[40:]
        cycle = cycle % self.resolution[0]
        # print(cycle)
        # print(sprite)
        if cycle in sprite:
            self.pixels[cycle] = "#"
        else:
            self.pixels[cycle] = "."

    def __getitem__(self, index):
        return self.pixels[index]

    def __setitem__(self, index, val):
        self.pixels[index] = val

    def __rich_console__(self, console, options):
        start = 0
        end = self.resolution[0]
        for row in range(self.resolution[1]):
            yield "".join(self.drawn[start:end])
            start += self.resolution[0]
            end += self.resolution[0]


class CPU:
    cycle: int = 1
    x: int = 1

    def __init__(self):
        self.crt = CRT()
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
    with open("day-10-input.txt", "r") as f:
        commands = [line.strip("\n").split(" ") for line in f.readlines()]
        for idx, command in enumerate(commands):
            if command[0] == "noop":
                commands[idx] = noop()
            elif command[0] == "addx":
                commands[idx] = addx(int(command[1]))

    cpu = CPU()
    for command in commands:
        cpu.eval(command)
    cycles = [i + 1 for i in range(220)][19::40]
    answer = 0
    for cycle in cycles:
        answer += cpu.history[cycle] * cycle
    print(answer)
    print(cpu.crt)
