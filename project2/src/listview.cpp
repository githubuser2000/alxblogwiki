#include "listview.h"
#include "html.h"
#include "pdf_template.h"
#include "query.h"
#include "media_view.h"
#include "csv.h"
#include "renderer.h"

#include <filesystem>
#include <vector>
#include <string>
#include <unordered_set>
#include <random>
#include <algorithm>
#include <iostream>
#include <fstream>

namespace fs = std::filesystem;

const fs::path DOCROOT = "/var/www/docs";

static std::string lower_ext(const std::string& filename) {
    std::string ext = fs::path(filename).extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(), [](unsigned char c){ return std::tolower(c); });
    return ext;
}

static void send_file(const std::string& filename) {
    fs::path path = DOCROOT / filename;
    if (!fs::exists(path) || !fs::is_regular_file(path)) {
        std::cout << "Status: 404 Not Found\r\nContent-Type: text/plain\r\n\r\nFile not found";
        return;
    }
    std::string ext = lower_ext(filename);
    std::string mime = "application/octet-stream";
    if (ext == ".pdf") mime = "application/pdf";
    else if (ext == ".mp4") mime = "video/mp4";
    else if (ext == ".txt" || ext == ".md" || ext == ".org" || ext == ".csv")
        mime = "text/plain; charset=utf-8";

    std::cout << "Content-Type: " << mime << "\r\n"
              << "Content-Length: " << fs::file_size(path) << "\r\n"
              << "Content-Disposition: inline; filename=\"" << filename << "\"\r\n\r\n";

    std::ifstream in(path, std::ios::binary);
    std::cout << in.rdbuf();
}

void render_list_view() {
    const char* qs = std::getenv("QUERY_STRING");
    if (qs) {
        std::string q(qs);
        if (q.rfind("raw=", 0) == 0) { send_file(q.substr(4)); return; }
        if (q.rfind("open=", 0) == 0) {
            std::string file = q.substr(5);
            std::string ext = lower_ext(file);
            if (ext == ".mp4") render_mp4_fullscreen(file);
            else if (ext == ".pdf") render_pdf_template_redirect(file);
            else if (ext == ".csv") render_csv_fullscreen(DOCROOT / file);
            else send_file(file);
            return;
        }
    }

    std::vector<std::string> files;
    static const std::unordered_set<std::string> allowed_ext = {".md",".org",".txt",".pdf",".mp4",".csv"};
    std::error_code ec;
    for (const auto& e : fs::directory_iterator(DOCROOT, ec)) {
        if (ec) break;
        if (!e.is_regular_file()) continue;
        std::string ext = lower_ext(e.path().filename().string());
        if (allowed_ext.find(ext) == allowed_ext.end()) continue;
        files.push_back(e.path().filename().string());
    }

    std::random_device rd;
    std::mt19937 rng(rd());
    std::shuffle(files.begin(), files.end(), rng);

    std::cout << "Content-Type: text/html; charset=utf-8\r\n\r\n"
              << "<!DOCTYPE html><html><head><meta charset='utf-8'><title>Dokumente</title>"
              << "<style>body{font-family:system-ui;margin:3em}</style></head><body><h1>Dokumente</h1><ul>";
    for (const auto& f : files)
        std::cout << "<li><a href='?open=" << html_escape(f) << "'>" << html_escape(f) << "</a></li>";
    std::cout << "</ul></body></html>";
}
