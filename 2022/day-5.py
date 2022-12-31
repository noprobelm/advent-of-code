import re
import numpy as np


class Crates:
    """
    Container for Crates and their positions based on the text input from 'day-5-input.txt', which is the puzzle input
    for day 5 of the 2022 'Advent of Code' series. Crates can be moved using the 'move' method, which references the
    procedure embedded in the puzzle input.

    Attributes
    -------
    layout : dict[int, list[str]]
        The layout of crates as they're currently stacked. Keys are the stack number, values are the bottom-to-top
        order of the crates.
    procedure : list[dict[str, int]]
        The iterative procedure the crane will follow. Keys for each dict are 'to', 'from', and 'qty'
    message : str
        The secret message revealed after all crates have been moved. This is the puzzle's answer.

    Methods
    -------
    reset()
        Resets the crates to their original positions.
    format_procedure(procedure: list[str])
        Static method: Used to translate the list of str instructions into a list of dicts structure
    format_layout(layout: list[str])
        Static method: Used to translate the unformatted layout of the crates into a dict structure
    move(model: str)
        Executes the procedure. 'Model' is the crane model to use for the method's logic (i.e. '9000' or '9001')
    """
    def __init__(self):
        self.layout = None
        self.procedure = None
        self.reset()

    def reset(self):
        """Resets the crates to their original positions"""
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
        """Used to translate the list of str instructions into a list of dicts structure

        Parameters
        -------
        procedure : list[str]
            List of the str-type instructions for the crane to carry out.

        Returns
        -------
        procedure_formatted : list[dict[str, int]]
            List of dicts of discrete categories to their respective int values
        """
        procedure_formatted = []
        for line in procedure:
            matches = [int(match) for match in re.findall(r"\d+", line)]
            instructions = {"qty": matches[0], "from": matches[1], "to": matches[2]}
            procedure_formatted.append(instructions)
        return procedure_formatted

    @staticmethod
    def format_layout(layout):
        """Used to translate the unformatted layout of the crates into a dict structure

        Parameters
        -------
        layout : list[str]
            List of strings with the original crate position layout embedded.

        Returns
        -------
        layout_formatted : dict[int, list[str]]
            dict of crate stack numbers to the bottom-to-top ordering
        """
        layout_formatted = {}
        for idx, line in enumerate(layout):
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
            elif re.match(r"\w", char):
                layout_formatted[stack].append(char)

        return layout_formatted


if __name__ == '__main__':
    crates = Crates()
    crates.move(model="9000")
    print(f"The hidden message with CrateMover 9000 is {crates.message}")
    crates.reset()

    crates = Crates()
    crates.move(model="9001")
    print(f"The hidden message with CrateMover 9001 is {crates.message}")
