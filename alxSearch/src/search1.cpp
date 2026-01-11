#include <iostream>
#include <fstream>
#include <filesystem>
#include <regex>
#include <unordered_map>

namespace fs = std::filesystem;

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <directory>\n";
        return 1;
    }

    fs::path dir = argv[1];
    std::regex token_regex(R"([A-Z][a-z]*)"); // Worttoken-Regel

    std::unordered_map<std::string, int> global_count;
    std::vector<std::tuple<std::string, std::string, int, int>> csv_rows;

    for (auto& p: fs::recursive_directory_iterator(dir)) {
        if (!fs::is_regular_file(p)) continue;
        auto size = fs::file_size(p);
        if (size > 500 * 1024) continue; // >500 KB

        std::ifstream file(p);
        if (!file) continue;

        std::unordered_map<std::string, int> file_count;
        std::string line;
        while (std::getline(file, line)) {
            for (std::sregex_iterator it(line.begin(), line.end(), token_regex), end_it; it != end_it; ++it) {
                std::string token = it->str();
                ++file_count[token];
                ++global_count[token];
            }
        }

        for (auto& [token, count] : file_count) {
            csv_rows.emplace_back(token, p.filename().string(), 0, count); // 0 als Platzhalter für global_count
        }
    }

    // Update globale Häufigkeit in CSV
    for (auto& row : csv_rows) {
        std::get<2>(row) = global_count[std::get<0>(row)];
    }

    // CSV schreiben
    std::ofstream csv("worttokens.csv");
    csv << "Token,Dateiname,HäufigkeitGesamt,HäufigkeitDatei\n";
    for (auto& row : csv_rows) {
        csv << std::get<0>(row) << "," 
            << std::get<1>(row) << ","
            << std::get<2>(row) << ","
            << std::get<3>(row) << "\n";
    }

    std::cout << "CSV erzeugt: worttokens.csv\n";
}
