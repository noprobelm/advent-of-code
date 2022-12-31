import re
import numpy as np


class Crates:
    def __init__(self):
        self.layout = None
        self.procedure = None
        self.reset()

    def reset(self):
        with open("day-5-input.txt", "r") as f:
            self.layout = []
            self.procedure = []
            for line in f.readlines():
                if re.match(r"^\W", line):
                    self.layout.append(line)
                else:
                    line = line.strip()
                    self.procedure.append(line)

        self.layout = self.format_layout(self.layout)
        self.procedure = self.format_procedure(self.procedure)

    @property
    def message(self):
        message = "".join([self.layout[stack][-1] for stack in range(1, 10)])
        return message

    def move(self, model):
        for instruction in self.procedure:
            if model == "9000":
                for i in range(instruction["qty"]):
                    self.layout[instruction["to"]].append(self.layout[instruction["from"]].pop())
            elif model == "9001":
                selected = self.layout[instruction["from"]][-instruction["qty"] :]
                for item in selected:
                    self.layout[instruction["to"]].append(item)
                    self.layout[instruction["from"]].pop()

    @staticmethod
    def format_procedure(procedure):
        procedure_formatted = []
        for line in procedure:
            matches = [int(match) for match in re.findall(r"\d+", line)]
            instructions = {"qty": matches[0], "from": matches[1], "to": matches[2]}
            procedure_formatted.append(instructions)
        return procedure_formatted

    @staticmethod
    def format_layout(layout):
        layout_formatted = {}

        for idx, line in enumerate(layout):
            line = re.sub("\n", " ", line)
            line = re.sub(r"[\[\]]", r" ", line)
            line = line.ljust(max([len(line) for line in layout]))
            line = list(line)
            layout[idx] = line

        layout = np.flip(np.transpose(np.array(layout)))
        mask = ~np.all(layout == " ", axis=1)[:, np.newaxis] & ~np.all(layout == " ", axis=0)
        layout = layout[mask]
        layout = list(layout)
        layout = [idx for idx in layout if idx != " "]
        for char in layout:
            if re.match(r"\d", char):
                stack = int(char)
                layout_formatted[stack] = []
            else:
                layout_formatted[stack].append(char)

        return layout_formatted


crates = Crates()
crates.move(model="9000")
print(f"The hidden message with CrateMover 9000 is {crates.message}")
crates.reset()

crates = Crates()
crates.move(model="9001")
print(f"The hidden message with CrateMover 9001 is {crates.message}")
