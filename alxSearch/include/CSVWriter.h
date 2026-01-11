#pragma once
#include "TokenExtractor.h"
#include <string>
#include <vector>

class CSVWriter {
public:
    static void writeCSV(const std::string& filename, const std::vector<TokenInfo>& tokens);
};
