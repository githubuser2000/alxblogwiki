#include "csv.h"
#include "html.h"
#include <fstream>
#include <iostream>
#include <vector>
#include <string>

static std::vector<std::string> parse_csv_line(const std::string& line, char sep) {
    std::vector<std::string> cells;
    std::string cell;
    bool in_quotes = false;

    for (size_t i = 0; i < line.size(); ++i) {
        char c = line[i];
        if (c == '"') {
            if (in_quotes && i + 1 < line.size() && line[i + 1] == '"') {
                cell += '"'; ++i;
            } else {
                in_quotes = !in_quotes;
            }
        } else if (c == sep && !in_quotes) {
            cells.push_back(cell);
            cell.clear();
        } else {
            cell += c;
        }
    }
    cells.push_back(cell);
    return cells;
}

void render_csv_fullscreen(const std::filesystem::path& file) {
    std::ifstream in(file);
    if (!in) { render_error("CSV-Datei konnte nicht geöffnet werden"); return; }

    std::string line;
    std::cout << "<style>"
                 "body{margin:0;padding:2em}"
                 "table{border-collapse:collapse;width:100%}"
                 "td,th{border:1px solid #888;padding:.3em;font-family:monospace;white-space:pre}"
                 "tr:nth-child(even){background:#f6f6f6}"
              "</style><table>";

    char sep = ';';
    if (std::getline(in, line)) {
        if (line.find(',') != std::string::npos && line.find(';') == std::string::npos) sep = ',';
        auto cells = parse_csv_line(line, sep);
        std::cout << "<tr>";
        for (auto& c : cells) std::cout << "<th>" << html_escape(c) << "</th>";
        std::cout << "</tr>";
    }

    while (std::getline(in, line)) {
        auto cells = parse_csv_line(line, sep);
        std::cout << "<tr>";
        for (auto& c : cells) std::cout << "<td>" << html_escape(c) << "</td>";
        std::cout << "</tr>";
    }
    std::cout << "</table>";
}
