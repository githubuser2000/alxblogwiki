#include "TokenExtractor.h"
#include <regex>
#include <fstream>
#include <sstream>
#include <set>
#include <map>

TokenExtractor::TokenExtractor() {}

const std::vector<TokenInfo>& TokenExtractor::getTokens() const {
    return csv_rows;
}

std::vector<TokenInfo> TokenExtractor::extractTokens(const std::string& text, const std::string& filename) const {
    std::vector<TokenInfo> tokens;
    std::regex inner_regex("([A-ZÄÖÜ][a-zäöüß]*)");

    std::istringstream iss(text);
    std::string word;
    size_t search_pos = 0;

    // Map um Token -> Liste der Positionen innerhalb dieser Datei
    std::map<std::string, std::vector<size_t>> token_positions;

    while (iss >> word) {
        size_t word_pos = text.find(word, search_pos);
        if (word_pos == std::string::npos) break;
        search_pos = word_pos + word.size();

        std::set<std::string> unique_tokens;
        auto words_begin = std::sregex_iterator(word.begin(), word.end(), inner_regex);
        auto words_end   = std::sregex_iterator();

        for (auto it = words_begin; it != words_end; ++it) {
            unique_tokens.insert(it->str());
        }

        for (const auto& tok : unique_tokens) {
            token_positions[tok].push_back(word_pos);
        }
    }

    // Baue TokenInfo-Strukturen
    for (const auto& [tok, positions] : token_positions) {
        tokens.push_back({tok, filename, 0, 0, positions});
    }

    return tokens;
}

void TokenExtractor::processFile(const std::string& filepath) {
    std::ifstream file(filepath);
    if (!file) return;

    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string text = buffer.str();

    auto file_tokens = extractTokens(text, filepath);

    std::unordered_map<std::string,int> file_count;
    for (auto &tok : file_tokens) {
        file_count[tok.token] = tok.positions.size();
        global_count[tok.token] += tok.positions.size();
    }

    for (auto &tok : file_tokens) {
        tok.file_count = file_count[tok.token];
        tok.global_count = global_count[tok.token];
        csv_rows.push_back(tok);
    }
}
