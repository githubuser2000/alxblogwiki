#include "listview.h"
#include "config.h"
#include "html.h"

#include <filesystem>
#include <vector>
#include <string>
#include <unordered_set>
#include <algorithm>
#include <random>
#include <iostream>
#include <fstream>
#include <cstdlib>

namespace fs = std::filesystem;

/* ------------------------------------------------------------
   Hilfsfunktion: Datei raw ausliefern (PDF / MP4 / Text)
------------------------------------------------------------ */
static void send_file(const std::string& filename)
{
    fs::path path = fs::path(DOCROOT) / filename;

    if (!fs::exists(path) || !fs::is_regular_file(path)) {
        std::cout
            << "Status: 404 Not Found\r\n"
            << "Content-Type: text/plain\r\n\r\n"
            << "File not found";
        return;
    }

    std::string ext = path.extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(),
                   [](unsigned char c){ return std::tolower(c); });

    std::string mime = "application/octet-stream";
    if (ext == ".pdf") mime = "application/pdf";
    else if (ext == ".mp4") mime = "video/mp4";
    else if (ext == ".txt" || ext == ".md" || ext == ".org" || ext == ".csv")
        mime = "text/plain; charset=utf-8";

    std::cout
        << "Content-Type: " << mime << "\r\n"
        << "Content-Length: " << fs::file_size(path) << "\r\n"
        << "Content-Disposition: inline; filename=\"" << filename << "\"\r\n"
        << "\r\n";

    std::ifstream in(path, std::ios::binary);
    std::cout << in.rdbuf();
}

/* ------------------------------------------------------------
   Hauptfunktion
------------------------------------------------------------ */
void render_list_view()
{
    /* ---------- QUERY_STRING auswerten ---------- */
    const char* qs = std::getenv("QUERY_STRING");
    if (qs && std::string(qs).rfind("file=", 0) == 0) {
        std::string filename = std::string(qs + 5);
        send_file(filename);
        return;
    }

    /* ---------- Dateiliste erzeugen ---------- */
    std::vector<std::string> files;

    static const std::unordered_set<std::string> allowed_ext = {
        ".md", ".org", ".txt", ".tx",
        ".pdf", ".mp4", ".rtf",
        ".mkv", ".csv"
    };

    std::error_code ec;

    for (const fs::directory_entry& e :
         fs::directory_iterator(DOCROOT, ec))
    {
        if (ec) break;
        if (!e.is_regular_file(ec)) continue;

        std::string ext = e.path().extension().string();
        std::transform(ext.begin(), ext.end(), ext.begin(),
                       [](unsigned char c){ return std::tolower(c); });

        if (allowed_ext.find(ext) == allowed_ext.end())
            continue;

        files.emplace_back(e.path().filename().string());
    }

    /* ---------- Zufällige Reihenfolge ---------- */
    std::random_device rd;
    std::mt19937 rng(rd());
    std::shuffle(files.begin(), files.end(), rng);

    /* ---------- HTML ausgeben ---------- */
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        << "<title>Dokumente</title>"
        << "<style>body{font-family:system-ui;margin:3em}</style>"
        << "</head><body><h1>Dokumente</h1><ul>";

    for (const auto& f : files) {
        std::cout
            << "<li><a href='?file="
            << html_escape(f)
            << "'>"
            << html_escape(f)
            << "</a></li>";
    }

    std::cout << "</ul></body></html>";
}
