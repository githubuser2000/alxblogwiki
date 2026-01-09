#include "listview.h"
#include "config.h"
#include "html.h"
#include <filesystem>
#include <vector>
#include <algorithm>
#include <iostream>

void render_list_view() {
    std::vector<std::string> files;

    for (const auto& e : std::filesystem::directory_iterator(DOCROOT)) {
        if (!e.is_regular_file() || e.is_symlink()) continue;
        auto ext = e.path().extension().string();
        if (ext == ".md" || ext == ".org" ||
            ext == ".txt" || ext == ".tx" ||
            ext == ".csv")
            files.push_back(e.path().filename().string());
    }

    std::sort(files.begin(), files.end());

    std::cout <<
        "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        "<title>Dokumente</title>"
        "<style>body{font-family:system-ui;margin:3em}</style>"
        "</head><body><h1>Dokumente</h1><ul>";

    for (auto& f : files)
        std::cout << "<li><a href='?file=" << html_escape(f) << "'>"
                  << html_escape(f) << "</a></li>";

    std::cout << "</ul></body></html>";
}
