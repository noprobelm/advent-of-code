#include "PuzzleInput.h"
#include <filesystem>
#include <fstream>
#include <sstream>
#include <stdexcept>

namespace fs = std::filesystem;

PuzzleInput::PuzzleInput(const std::string& filename) {
    fs::path rootPath = fs::current_path();
    fs::path puzzlePath = rootPath / "../../data";
    fs::path filePath = puzzlePath / filename;

    std::ifstream file(filePath);
    if (!file) {
        throw std::runtime_error("Failed to open file: " + filePath.string());
    }

    std::ostringstream buffer;
    buffer << file.rdbuf();
    rawData = buffer.str();
    puzzleData = rawData;
    puzzleData.erase(puzzleData.find_last_not_of(" \n\r\t") + 1);
}

std::vector<std::string> PuzzleInput::lines() const {
    std::vector<std::string> result;
    std::istringstream stream(puzzleData);
    std::string line;

    while (std::getline(stream, line)) {
        result.push_back(line);
    }

    return result;
}

std::string PuzzleInput::str() const {
    return puzzleData;
}
