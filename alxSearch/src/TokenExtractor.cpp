#include "TokenExtractor.h"
#include <regex>
#include <fstream>
#include <sstream>
#include <set>

TokenExtractor::TokenExtractor() {}

// Tokenisierung: Leerzeichen + Großbuchstaben inkl. deutsche Umlaute
std::vector<std::string> TokenExtractor::extractTokens(const std::string& line) const {
    std::vector<std::string> tokens;

    std::istringstream iss(line);
    std::string word;
    while (iss >> word) {
        // Regex für Großbuchstaben-Token inkl. deutsche Buchstaben
        // Hinweis: UTF-8 direkt in std::string, libc++ kompatibel
        std::regex inner_regex("([A-ZÄÖÜ][a-zäöüß]*)");
        std::set<std::string> unique_tokens;

        auto words_begin = std::sregex_iterator(word.begin(), word.end(), inner_regex);
        auto words_end   = std::sregex_iterator();

        for (auto it = words_begin; it != words_end; ++it) {
            unique_tokens.insert(it->str());
        }

        // Einfügen der einzigartigen Tokens
        tokens.insert(tokens.end(), unique_tokens.begin(), unique_tokens.end());
    }

    return tokens;
}

void TokenExtractor::processFile(const std::string& filepath) {
    std::ifstream file(filepath);
    if (!file) return;

    std::unordered_map<std::string,int> file_count;
    std::string line;

    while (std::getline(file,line)) {
        auto tokens = extractTokens(line);
        for (const auto& token : tokens) {
            ++file_count[token];
            ++global_count[token];
        }
    }

    for (auto& [token,count] : file_count) {
        csv_rows.push_back({token, filepath, 0, count});
    }
}

void TokenExtractor::writeCSV(const std::string& csvfile) const {
    std::ofstream csv(csvfile);
    csv << "Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei\n";
    for (auto row : csv_rows) {
        row.global_count = global_count.at(row.token);
        csv << row.token << "," << row.filename << "," << row.global_count << "," << row.file_count << "\n";
    }
}
