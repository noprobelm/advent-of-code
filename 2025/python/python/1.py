puzzle_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".split("\n")

with open("../../data/1.txt", "r") as f:
    puzzle_input = f.readlines()


def part_1():
    answer = 0
    position = 50

    for line in puzzle_input:
        rotation = int(line[1:])
        position = position - rotation if line[0] == "L" else position + rotation

        position %= 100

        if position == 0:
            answer += 1

    return answer


def part_2():
    answer = 0
    position = 50

    for line in puzzle_input:
        rotation = int(line[1:])
        position = position - rotation if line[0] == "L" else position + rotation

        answer += abs(position // 100)
        if position <= 0:
            # If we rotate left and land on -200, we've actually passed '0' 3 times (i.e., abs(-200 // 100 = 2)), so add 1.
            if position % 100 == 0:
                answer += 1
            # If this is True, we started from 0 and thusly already counted it in the previous iteration (0 % 100 = 0).
            if rotation == abs(position):
                answer -= 1

        # Normalize the position to 0-99
        position %= 100

    return answer


if __name__ == "__main__":
    answer = part_1()
    print(answer)
    answer = part_2()
    print(answer)
