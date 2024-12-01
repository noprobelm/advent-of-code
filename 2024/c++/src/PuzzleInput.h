#ifndef PUZZLEINPUT_H
#define PUZZLEINPUT_H

#include <string>
#include <vector>

class PuzzleInput {
private:
    std::string rawData;
    std::string puzzleData;

public:
    explicit PuzzleInput(const std::string& filename);

    std::vector<std::string> lines() const;
    std::string str() const;
};

#endif // PUZZLEINPUT_H
