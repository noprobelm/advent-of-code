#!/usr/bin/env python3
from rich.console import Console
from dataclasses import dataclass

console = Console()


def main():
    with open("../../data/9.txt", "r") as f:
        moves = [line.strip() for line in f.readlines()]

    for idx, move in enumerate(moves):
        move = move.split(" ")
        moves[idx] = [move[0], int(move[1])]

    # Puzzle 1
    rope = Rope(num_knots=2)
    for move in moves:
        rope.move(move)

    print(f"Part 1: {len(rope.tail_history)}")

    # Puzzle 2
    rope = Rope(num_knots=10)
    for move in moves:
        rope.move(move)

    print(f"Part 2: {len(rope.tail_history)}")


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
            if axis == 0:
                self.knot_0 = Position(self.knot_0.x, self.knot_0.y + direction)
            elif axis == 1:
                self.knot_0 = Position(self.knot_0.x + direction, self.knot_0.y)

            for _ in range(1, self.num_knots):
                leader = getattr(self, f"knot_{_ - 1}")
                follower = getattr(self, f"knot_{_}")
                if abs(leader.y - follower.y) > 1 or abs(leader.x - follower.x) > 1:
                    if leader.y > follower.y:
                        follower = Position(follower.x, follower.y + 1)
                    elif leader.y < follower.y:
                        follower = Position(follower.x, follower.y - 1)
                    if leader.x > follower.x:
                        follower = Position(follower.x + 1, follower.y)
                    elif leader.x < follower.x:
                        follower = Position(follower.x - 1, follower.y)

                setattr(self, f"knot_{_}", follower)

            self.tail_history.add(getattr(self, f"knot_{self.num_knots-1}"))


if __name__ == "__main__":
    main()
