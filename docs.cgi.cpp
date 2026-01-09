#include <iostream>
#include <filesystem>
#include <fstream>
#include <vector>
#include <algorithm>
#include <cstdlib>
#include <cstdio>

namespace fs = std::filesystem;
static const fs::path DOCROOT{"/var/www/docs"};

/* ---------------- HTML escape ---------------- */
std::string esc(const std::string& s) {
    std::string r;
    for (char c : s) {
        switch (c) {
            case '&': r += "&amp;"; break;
            case '<': r += "&lt;"; break;
            case '>': r += "&gt;"; break;
            case '"': r += "&quot;"; break;
            default:  r += c;
        }
    }
    return r;
}

/* ------------- Query: file= ------------------ */
std::string query_file() {
    const char* qs = std::getenv("QUERY_STRING");
    if (!qs) return {};
    std::string q(qs);
    auto p = q.find("file=");
    if (p == std::string::npos) return {};
    std::string v = q.substr(p + 5);
    if (v.find("..") != std::string::npos) return {};
    if (v.find('/')  != std::string::npos) return {};
    return v;
}

/* -------- pandoc render (md/org) ------------- */
bool render_with_pandoc(const fs::path& file) {
    std::string cmd =
        "pandoc --standalone --from=" +
        std::string(file.extension() == ".org" ? "org" : "markdown") +
        " --to=html5 \"" + file.string() + "\"";

    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe) return false;

    char buf[4096];
    while (fgets(buf, sizeof(buf), pipe))
        std::cout << buf;

    pclose(pipe);
    return true;
}

/* ------------------- main -------------------- */
int main() {
    /* ---- HTTP + Security Headers ---- */
    std::cout <<
        "Content-Type: text/html; charset=utf-8\n"
        "X-Content-Type-Options: nosniff\n"
        "Referrer-Policy: no-referrer\n"
        "Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'\n\n";

    std::string file = query_file();

    /* ================= FILE VIEW ================= */
    if (!file.empty()) {
        fs::path p = DOCROOT / file;

        if (!fs::exists(p) || !fs::is_regular_file(p) || fs::is_symlink(p)) {
            std::cout << "<h1>404</h1>";
            return 0;
        }

        std::string ext = p.extension().string();
        if (ext != ".md" && ext != ".org" &&
            ext != ".txt" && ext != ".tx") {
            std::cout << "<h1>Invalid file</h1>";
            return 0;
        }

        std::cout <<
            "<!DOCTYPE html><html><head><meta charset='utf-8'>"
            "<title>" << esc(file) << "</title>"
            "<style>"
            "body{margin:0;padding:4em;font-family:serif;max-width:80ch}"
            "pre{white-space:pre-wrap}"
            "a{position:fixed;top:1em;left:1em}"
            "</style></head><body>"
            "<a href='?'>← zurück</a>";

        if (ext == ".md" || ext == ".org") {
            if (!render_with_pandoc(p)) {
                std::cout << "<pre>";
                std::ifstream in(p);
                std::string line;
                while (std::getline(in, line))
                    std::cout << esc(line) << "\n";
                std::cout << "</pre>";
            }
        } else {
            std::cout << "<pre>";
            std::ifstream in(p);
            std::string line;
            while (std::getline(in, line))
                std::cout << esc(line) << "\n";
            std::cout << "</pre>";
        }

        std::cout << "</body></html>";
        return 0;
    }

    /* ================= LIST VIEW ================= */
    std::vector<std::string> files;

    for (const auto& e : fs::directory_iterator(DOCROOT)) {
        if (!e.is_regular_file() || e.is_symlink()) continue;
        auto ext = e.path().extension().string();
        if (ext == ".md" || ext == ".org" ||
            ext == ".txt" || ext == ".tx")
            files.push_back(e.path().filename().string());
    }

    std::sort(files.begin(), files.end());

    std::cout <<
        "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        "<title>Dokumente</title>"
        "<style>"
        "body{font-family:system-ui;margin:3em}"
        "li{margin:.4em 0}"
        "</style></head><body>"
        "<h1>Dokumente</h1><ul>";

    for (auto& f : files)
        std::cout << "<li><a href='?file=" << esc(f) << "'>"
                  << esc(f) << "</a></li>";

    std::cout << "</ul></body></html>";
    return 0;
}
