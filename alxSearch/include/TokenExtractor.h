#pragma once
#include <string>
#include <unordered_map>
#include <vector>

struct TokenInfo {
    std::string token;
    std::string filename;
    int global_count;
    int file_count;
    std::vector<size_t> positions; // Alle Positionen in der Datei
};

class TokenExtractor {
public:
    TokenExtractor();
    void processFile(const std::string& filepath);
    const std::vector<TokenInfo>& getTokens() const;

private:
    std::unordered_map<std::string, int> global_count;
    std::vector<TokenInfo> csv_rows;

    std::vector<TokenInfo> extractTokens(const std::string& text, const std::string& filename) const;
};
