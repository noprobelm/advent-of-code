import re


def main():
    with open("../../data/1.txt", "r") as f:
        elves = [re.split("\n", elf) for elf in re.split(r"\n\n", f.read())][:-1]
        for elf in elves:
            for i, meal in enumerate(elf):
                elf[i] = int(meal)

    calories = list(sorted([sum(meals) for meals in elves], reverse=True))
    largest = calories[0]
    top_three_cumulative = sum(calories[:3])

    print(f"Puzzle 1: {largest}")
    print(f"Puzzle 2: {top_three_cumulative}")


if __name__ == "__main__":
    main()
