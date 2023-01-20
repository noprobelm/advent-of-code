#!/usr/bin/env python3
from rich.console import Console
from collections import namedtuple
from dataclasses import dataclass
from rich.pretty import pprint

console = Console()


@dataclass(frozen=True)
class Position:
    x: int = 0
    y: int = 0


class Rope:
    def __init__(self):
        self.movement_mapper = {
            "U": {"axis": 0, "direction": 1},
            "D": {"axis": 0, "direction": -1},
            "L": {"axis": 1, "direction": -1},
            "R": {"axis": 1, "direction": 1},
        }
        self.head, self.tail = Position(), Position()
        self.movement_history = {
            "head": {Position(0, 0): 1},
            "tail": {Position(0, 0): 1},
        }

    def move(self, instruction):
        axis = self.movement_mapper[instruction[0]]["axis"]
        direction = self.movement_mapper[instruction[0]]["direction"]
        magnitude = instruction[1]

        def update_head():
            if axis == 0:
                self.head = Position(self.head.x, self.head.y + direction)
            elif axis == 1:
                self.head = Position(self.head.x + direction, self.head.y)
            try:
                self.movement_history["head"][Position(self.head.x, self.head.y)] += 1
            except KeyError:
                self.movement_history["head"][Position(self.head.x, self.head.y)] = 1

        def update_tail():
            if abs(self.tail.x - self.head.x) > 1 or abs(self.tail.y - self.head.y) > 1:
                self.tail = head_old
            try:
                self.movement_history["tail"][Position(self.tail.x, self.tail.y)] += 1
            except KeyError:
                self.movement_history["tail"][Position(self.tail.x, self.tail.y)] = 1

        for _ in range(magnitude):
            head_old = Position(self.head.x, self.head.y)
            update_head()
            update_tail()


if __name__ == "__main__":
    with open("day-9-input.txt", "r") as f:
        moves = [line.strip() for line in f.readlines()]

    for idx, move in enumerate(moves):
        move = move.split(" ")
        moves[idx] = [move[0], int(move[1])]

    rope = Rope()
    for move in moves:
        rope.move(move)

    print(len(rope.movement_history["tail"]))
