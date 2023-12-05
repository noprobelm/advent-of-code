import re
import os
from . import PUZZLE_DIR


def main():
    with open(os.path.join(PUZZLE_DIR, "4.txt"), "r") as f:
        data = f.read().strip()
    lines = data.split("\n")

    answer_1 = part_1(lines)
    print(f"Part 1: {answer_1}")

    answer_2 = part_2(lines)
    print(f"Part 2: {answer_2}")


def part_1(lines: list[str]):
    answer = 0
    for line in lines:
        _, winning, draws = parse_card(line)
        matches = [n for n in draws if n in winning]
        if len(matches) == 0:
            points = 0
        else:
            points = 1
            for _ in range(len(matches) - 1):
                points *= 2
        answer += points
    return answer


def part_2(lines: list[str]):
    winnings = {}
    cards = []
    for line in lines:
        card_id, winning, draws = parse_card(line)
        winnings[card_id] = [n for n in draws if n in winning]
        cards.append(card_id)

    for card_id in cards:
        matches = winnings[card_id]
        for n in range(len(matches)):
            cards.append(card_id + n + 1)

    return len(cards)


def parse_card(s: str):
    split = re.split(r":\s+", s)
    card_id = int(re.search(r"\d+", split[0]).group())
    numbers = re.split(r"\|", split[1])
    winning_nums = re.findall(r"\d+", numbers[0])
    card_nums = re.findall(r"\d+", numbers[1])

    return (card_id, winning_nums, card_nums)
