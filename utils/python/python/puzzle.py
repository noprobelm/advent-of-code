import os


class Puzzle(list):
    def __init__(self, path: str | os.PathLike) -> None:
        with open(path, "r") as f:
            self._data = f.read().strip()

        super().__init__(self._data.split("\n"))

    def __str__(self) -> str:
        return self._data
