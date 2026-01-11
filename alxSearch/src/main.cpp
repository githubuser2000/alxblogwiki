#include "TokenExtractor.h"
#include "FileUtils.h"
#include "CSVWriter.h"
#include <iostream>

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <directory>\n";
        return 1;
    }

    std::string dir = argv[1];

    TokenExtractor extractor;
    auto files = FileUtils::listTextFiles(dir, 500*1024);

    for (auto &f : files) {
        extractor.processFile(f);
    }

    CSVWriter::writeCSV("worttokens.csv", extractor.getTokens());

    std::cout << "CSV erzeugt: worttokens.csv\n";
}
