with open("day-1-input.txt", "r") as f:
    calories = []
    elf = []
    for line in f.readlines():
        line = line.replace("\n", "")
        if line:
            elf.append(int(line))
        else:
            calories.append(elf)
            elf = []

calories = list(sorted([sum(foods) for foods in calories], reverse=True))
largest = calories[0]
top_three_cumulative = sum(calories[:3])

print(f"The elf with the most calories is carrying {largest} calories.")
print(
    f"The cumulative sum of the calories carried by the top three elves with the most calories is {top_three_cumulative}"
)
