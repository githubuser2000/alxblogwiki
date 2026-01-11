#pragma once
#include <string>
#include <unordered_map>
#include <vector>

struct TokenInfo {
    std::string token;
    std::string filename;
    int global_count;
    int file_count;
};

class TokenExtractor {
public:
    TokenExtractor();
    void processFile(const std::string& filepath);
    void writeCSV(const std::string& csvfile) const;
private:
    std::unordered_map<std::string, int> global_count;
    std::vector<TokenInfo> csv_rows;
};
