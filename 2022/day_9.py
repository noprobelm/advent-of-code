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
    movement_mapper = {
        "U": {"axis": 0, "direction": 1},
        "D": {"axis": 0, "direction": -1},
        "L": {"axis": 1, "direction": -1},
        "R": {"axis": 1, "direction": 1},
    }

    def __init__(self, num_knots: int):
        self.num_knots = num_knots
        for knot in range(num_knots):
            setattr(self, f"knot_{knot}", Position())
        self.tail_history = set({Position()})

    def move(self, move):
        axis = self.movement_mapper[move[0]]["axis"]
        direction = self.movement_mapper[move[0]]["direction"]
        magnitude = move[1]

        for move in range(magnitude):
            leader_old = Position(self.knot_0.x, self.knot_0.y)
            if axis == 0:
                self.knot_0 = Position(self.knot_0.x, self.knot_0.y + direction)
            elif axis == 1:
                self.knot_0 = Position(self.knot_0.x + direction, self.knot_0.y)

            for knot in range(1, self.num_knots):
                leader = getattr(self, f"knot_{knot - 1}")
                follower = getattr(self, f"knot_{knot}")
                if abs(follower.x - leader.x) > 1 or abs(follower.y - leader.y) > 1:
                    follower = Position(leader_old.x, leader_old.y)
                    setattr(self, f"knot_{knot}", follower)
                leader_old = Position(leader.x, leader.y)

            self.tail_history.add(getattr(self, f"knot_{self.num_knots-1}"))


if __name__ == "__main__":
    with open("day-9-input.txt", "r") as f:
        moves = [line.strip() for line in f.readlines()]

    for idx, move in enumerate(moves):
        move = move.split(" ")
        moves[idx] = [move[0], int(move[1])]

    # Puzzle 1
    rope = Rope(num_knots=2)
    for move in moves:
        rope.move(move)
    print(len(rope.tail_history))

    # Puzzle 2
    rope = Rope(num_knots=10)
    for move in moves:
        rope.move(move)
    print(len(rope.tail_history))
