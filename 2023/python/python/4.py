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
        split = re.split(r":\s+", line)
        numbers = re.split(r"\|", split[1])
        winning_numbers = re.findall(r"\d+", numbers[0])
        card_numbers = re.findall(r"\d+", numbers[1])

        num_found = 0
        for num in card_numbers:
            if num in winning_numbers:
                num_found += 1
        if num_found == 0:
            points = 0
        else:
            points = 1
            for _ in range(num_found - 1):
                points *= 2
        answer += points
    return answer


def part_2(lines: list[str]):
    answer = 0
    cards = {}
    rounds = [[]]
    for card in lines:
        card_id, winning, nums = parse_card(card)
        cards[card_id] = {
            "winning": winning,
            "nums": nums,
        }
        rounds[0].append(card_id)

    current = 0
    while True:
        rounds.append([])
        for card_id in rounds[current]:
            num_found = 0
            winning = cards[card_id]["winning"]
            nums = cards[card_id]["nums"]
            for num in nums:
                if num in winning:
                    num_found += 1
            for k in range(num_found):
                rounds[current + 1].append(card_id + k + 1)
        if len(rounds[current + 1]) == 0:
            break
        current += 1

    return sum([len(r) for r in rounds])


def parse_card(s: str):
    split = re.split(r":\s+", s)
    card_id = int(re.search(r"\d+", split[0]).group())
    numbers = re.split(r"\|", split[1])
    winning_nums = re.findall(r"\d+", numbers[0])
    card_nums = re.findall(r"\d+", numbers[1])

    return (card_id, winning_nums, card_nums)
