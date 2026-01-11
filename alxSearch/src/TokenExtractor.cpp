#include "TokenExtractor.h"
#include "FileUtils.h"
#include <regex>
#include <fstream>

TokenExtractor::TokenExtractor() {}

void TokenExtractor::processFile(const std::string& filepath) {
    std::regex token_regex(R"([A-Z][a-z]*)");
    std::ifstream file(filepath);
    if(!file) return;

    std::unordered_map<std::string,int> file_count;
    std::string line;
    while(std::getline(file,line)) {
        for(std::sregex_iterator it(line.begin(), line.end(), token_regex), end_it; it!=end_it;++it) {
            std::string token = it->str();
            ++file_count[token];
            ++global_count[token];
        }
    }

    for(auto& [token, count] : file_count) {
        csv_rows.push_back({token, filepath, 0, count}); // global_count wird später gesetzt
    }
}

void TokenExtractor::writeCSV(const std::string& csvfile) const {
    std::ofstream csv(csvfile);
    csv << "Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei\n";
    for(auto row : csv_rows) {
        row.global_count = global_count.at(row.token);
        csv << row.token << "," << row.filename << "," << row.global_count << "," << row.file_count << "\n";
    }
}
