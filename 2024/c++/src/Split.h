#ifndef SPLIT_H
#define SPLIT_H

#include <string>
#include <vector>
#include <regex>

// Split using a regex (e.g., multiple spaces)
inline std::vector<std::string> split(const std::string &s, const std::string &delimRegex) {
    std::vector<std::string> elems;
    std::regex regex(delimRegex);
    std::sregex_token_iterator it(s.begin(), s.end(), regex, -1);
    std::sregex_token_iterator end;
    for (; it != end; ++it) {
        if (!it->str().empty()) { // Ignore empty results
            elems.push_back(it->str());
        }
    }
    return elems;
}

#endif // SPLIT_H
