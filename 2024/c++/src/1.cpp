#include "PuzzleInput.h"
#include "Split.h"
#include <cstdint>
#include <iostream>
#include <vector>

int part_1(std::vector<int> left, std::vector<int> right) {
  sort(left.begin(), left.end(), [](int a, int b) { return a >= b; });
  sort(right.begin(), right.end(), [](int a, int b) { return a >= b; });

  int distance = 0;

  for (std::size_t index = 0; index < left.size(); index++) {
    int left_n = left[index];
    int right_n = right[index];
    if (left_n > right_n) {
      distance += left_n - right_n;
    } else {
      distance += right_n - left_n;
    }
  }

  return distance;
}

int part_2(std::vector<int> left, std::vector<int> right) {
  int count = 0;
  int matches = 0;
  for (int left_n : left) {
    for (int right_n : right) {
      if (left_n == right_n) {
        matches += 1;
      }
    }
    count += left_n * matches;
    matches = 0;
  }

  return count;
}

int main() {
  try {
    PuzzleInput puzzle("1.txt");
    std::vector<int> left;
    std::vector<int> right;

    for (std::string line : puzzle.lines()) {
      std::vector<std::string> nums = split(line, "\\s+");
      left.push_back(std::stoul(nums[0]));
      right.push_back(std::stoul(nums[1]));
    }

    int distance = part_1(left, right);
    std::cout << "Part 1: " << distance << std::endl;

    int count = part_2(left, right);
    std::cout << "Part 2: " << count << std::endl;

  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
