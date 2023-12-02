def main():
    with open("../../data/6.txt", "r") as f:
        data = f.read()

    marker_4 = get_marker(data, 4)
    marker_14 = get_marker(data, 14)

    print(f"Part 1: {marker_4}")
    print(f"Part 2: {marker_14}")


def get_marker(data: str, group_size: int) -> int:
    for idx in range(len(data) - group_size):
        group = data[idx : idx + group_size]
        for pos, char in enumerate(group):
            if group.count(char) > 1:
                break
            elif pos == len(group) - 1:
                marker_pos = idx + group_size
                return marker_pos


if __name__ == "__main__":
    main()
