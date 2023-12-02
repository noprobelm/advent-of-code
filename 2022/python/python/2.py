def part_1(rounds):
    strategy = Strategy(rounds)
    strategy.decrypt(key="moves")
    strategy.calculate_score()
    return strategy.score


def part_2(rounds):
    strategy = Strategy(rounds)
    strategy.decrypt(key="outcomes")
    strategy.calculate_score()
    return strategy.score


def main():
    with open("../../data/2.txt", "r") as f:
        rounds = [[line[0], line[-2]] for line in f.readlines()]

    part_1_answer = part_1(rounds)
    print(f"Part 1: {part_1_answer}")

    part_2_answer = part_2(rounds)
    print(f"Part 2: {part_2_answer}")


class Move:
    def __eq__(self, other):
        if isinstance(other, type(self)):
            return True


class Rock(Move):
    points = 1

    def __lt__(self, other):
        if type(other) == Paper:
            return True
        else:
            return False

    def __gt__(self, other):
        if type(other) == Scissors:
            return True
        else:
            return False


class Paper(Move):
    points = 2

    def __lt__(self, other):
        if type(other) == Scissors:
            return True
        elif type(other) == Rock:
            return False

    def __gt__(self, other):
        if type(other) == Rock:
            return True
        else:
            return False


class Scissors(Move):
    points = 3

    def __lt__(self, other):
        if type(other) == Rock:
            return True
        elif type(other) == Paper:
            return False

    def __gt__(self, other):
        if type(other) == Paper:
            return True
        else:
            return False


class Strategy:
    def __init__(self, rounds: list[list]):
        self.rounds = rounds
        self.moves = []
        self.score = 0

    def decrypt(self, key):
        for match in self.rounds:
            if match[0] == "A":
                opponent_move = Rock()
            elif match[0] == "B":
                opponent_move = Paper()
            elif match[0] == "C":
                opponent_move = Scissors()

            if match[1] == "X":
                if key == "moves":
                    counter_move = Rock()
                elif key == "outcomes":
                    for selected in [Rock, Paper, Scissors]:
                        if opponent_move > selected():
                            counter_move = selected()

            elif match[1] == "Y":
                if key == "moves":
                    counter_move = Paper()
                elif key == "outcomes":
                    for selected in [Rock, Paper, Scissors]:
                        if opponent_move == selected():
                            counter_move = selected()

            else:
                if key == "moves":
                    counter_move = Scissors()
                elif key == "outcomes":
                    for selected in [Rock, Paper, Scissors]:
                        if opponent_move < selected():
                            counter_move = selected()

            self.moves.append([opponent_move, counter_move])

    def calculate_score(self):
        for move in self.moves:
            if move[0] < move[1]:
                self.score += move[1].points + 6
            elif move[0] == move[1]:
                self.score += move[1].points + 3
            elif move[0] > move[1]:
                self.score += move[1].points


if __name__ == "__main__":
    main()
