import os

ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PUZZLE_PATH = os.path.join(ROOT_PATH, "../../data")

class PuzzleInput:
    def __init__(self, filename: str):
        with open(os.path.join(PUZZLE_PATH, filename), 'r') as f:
           self._raw = f.read()
           self.puzzle = self._raw.strip()

    def lines(self) -> list[str]:
        return self.puzzle.split("\n")

    def str(self) -> str:
        return self.puzzle
