SAMPLE = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""


def main():
    with open(f"../../data/13.txt", "r") as f:
        input_str = f.read()

    packets = get_packets(input_str)
    print("PUZZLE 1")
    part_1 = solve_puzzle_1(packets)
    print("PUZZLE 2")
    solve_puzzle_2(packets)


class Packet:
    def __init__(self, data: list):
        self.data = data

    def __str__(self):
        return str(self.data)

    def __repr__(self):
        return str(self.data)

    def __lt__(self, other: "Packet"):
        if not isinstance(other, Packet):
            raise TypeError("Incomparable types")
        for i in range(max([len(self.data), len(other.data)])):
            if i + 1 > len(self.data):
                return True
            elif i + 1 > len(other.data):
                return False
            elif isinstance(self.data[i], list) and isinstance(other.data[i], list):
                if self.data[i] == other.data[i]:
                    continue
                return Packet(self.data[i]) < Packet(other.data[i])
            elif isinstance(self.data[i], int) and isinstance(other.data[i], int):
                if self.data[i] == other.data[i]:
                    continue
                return self.data[i] < other.data[i]
            elif isinstance(self.data[i], int) and isinstance(other.data[i], list):
                if [self.data[i]] == other.data[i]:
                    continue
                return Packet([self.data[i]]) < Packet(other.data[i])

            elif isinstance(other.data[i], int) and isinstance(self.data[i], list):
                if self.data[i] == [other.data[i]]:
                    continue
                return Packet(self.data[i]) < Packet([other.data[i]])


def get_packets(input_str: str) -> list:
    packets = []
    input_str = input_str.split("\n")
    for line in input_str:
        if line.startswith("["):
            packet = eval(f"list({line})")
            packet = Packet(packet)
            packets.append(packet)

    return packets


def solve_puzzle_1(packets):
    pairs = []
    for num, packet in enumerate(packets):
        num += 1
        if num % 2 == 1:
            pairs.append([packet, packets[num]])

    ordered_idx = []
    for pair in pairs:
        ordered_idx.append(pair[0] < pair[1])
    print(
        f"The sum of the index of each correctly-ordered packet pair is {sum([i + 1 for i in range(len(ordered_idx)) if ordered_idx[i] is True])}",
        end="\n\n",
    )


def solve_puzzle_2(packets):
    first_divider = Packet([[2]])
    second_divider = Packet([[6]])
    packets.extend([first_divider, second_divider])
    packets = list(sorted(packets))

    product = 1
    for num, packet in enumerate(packets):
        if packet.data == first_divider.data or packet.data == second_divider.data:
            product *= num + 1

    print(f"The product of each packet divider's index is {product}")


if __name__ == "__main__":
    main()
