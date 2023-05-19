def get_marker(group_size: int) -> int:
    for idx in range(len(buffer) - group_size):
        group = buffer[idx : idx + group_size]
        for pos, char in enumerate(group):
            if group.count(char) > 1:
                break
            elif pos == len(group) - 1:
                marker_pos = idx + group_size
                return marker_pos


if __name__ == "__main__":
    with open("day-6-input.txt", "r") as f:
        buffer = f.read()

    marker_4 = get_marker(4)
    marker_14 = get_marker(14)

    print(f"'start-of-packet-marker' for 4 character groups found in position {marker_4}")
    print(f"'start-of-packet-marker' for 14 character groups found in position {marker_14}")
