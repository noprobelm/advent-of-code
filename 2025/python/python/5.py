from __future__ import annotations
from dataclasses import dataclass
from functools import reduce

fresh = [(3, 5), (10, 14), (16, 20), (12, 18)]
available = [1, 5, 8, 11, 17, 32]

with open("../../data/5.txt", "r") as f:
    fresh = []
    available = []
    on_fresh = True
    for line in f.readlines():
        line = line.strip()
        if line == "":
            on_fresh = False
            continue
        if on_fresh:
            split = line.split("-")
            fresh.append((int(split[0]), int(split[1])))
        else:
            available.append(int(line))
    puzzle_input = [line.strip() for line in f.readlines()]


def part_1():
    answer = 0
    for ingredient in available:
        ingredient = int(ingredient)
        for start, stop in fresh:
            if ingredient >= start and ingredient <= stop:
                answer += 1
                break

    return answer


def part_2():
    consolidation_performed = True
    while consolidation_performed:
        consolidation_performed = False
        for i, (start, stop) in enumerate(fresh):
            for k, (other_start, other_stop) in enumerate(fresh):
                if i == k:
                    continue
                if (
                    # Range is fully encompassed by other
                    (start <= other_start and stop >= other_stop)
                    # Range starts on or before other start, stops on or before other stop
                    or (
                        start <= other_start
                        and stop >= other_start
                        and stop <= other_stop
                    )
                    # Range starts on or after other start, stops on or after other stop
                    or (start <= other_stop and stop >= other_stop)
                ):
                    fresh[k] = (min((start, other_start)), max((stop, other_stop)))
                    consolidation_performed = True
            if consolidation_performed:
                del fresh[i]
                break

    return reduce(lambda acc, r: acc + r[1] - r[0] + 1, fresh, 0)


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
