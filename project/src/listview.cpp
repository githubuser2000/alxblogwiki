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
   Hilfsfunktion: Extension normalisieren
------------------------------------------------------------ */
static std::string lower_ext(const std::string& filename)
{
    std::string ext = fs::path(filename).extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(),
                   [](unsigned char c){ return std::tolower(c); });
    return ext;
}

/* ------------------------------------------------------------
   RAW-Datei ausliefern (PDF / MP4 / Text / etc.)
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

    std::string ext = lower_ext(filename);

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
   MP4 Vollbild-HTML
------------------------------------------------------------ */
static void render_mp4_fullscreen(const std::string& filename)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
        << "<title>Video</title>"
        << "<style>"
        << "html,body{margin:0;width:100%;height:100%;background:black;overflow:hidden}"
        << "video{width:100%;height:100%;object-fit:contain}"
        << "</style></head><body>"
        << "<video controls autoplay muted playsinline>"
        << "<source src='?raw=" << html_escape(filename) << "' type='video/mp4'>"
        << "</video></body></html>";
}

/* ------------------------------------------------------------
   PDF Vollbild-HTML
------------------------------------------------------------ */
static void render_pdf_fullscreen(const std::string& filename)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
        << "<title>PDF</title>"
        << "<style>"
        << "html,body{margin:0;width:100%;height:100%;overflow:hidden;background:#111}"
        << "iframe{width:100%;height:100%;border:0}"
        << "</style></head><body>"
        << "<iframe src='?raw=" << html_escape(filename) << "'></iframe>"
        << "</body></html>";
}

/* ------------------------------------------------------------
   Hauptfunktion
------------------------------------------------------------ */
void render_list_view()
{
    /* ---------- QUERY_STRING auswerten ---------- */
    const char* qs = std::getenv("QUERY_STRING");
    if (qs) {
        std::string q(qs);

        /* RAW-Auslieferung */
        if (q.rfind("raw=", 0) == 0) {
            send_file(q.substr(4));
            return;
        }

        /* Öffnen (HTML-View oder RAW je nach Typ) */
        if (q.rfind("open=", 0) == 0) {
            std::string filename = q.substr(5);
            std::string ext = lower_ext(filename);

            if (ext == ".mp4")
                render_mp4_fullscreen(filename);
            else if (ext == ".pdf")
                render_pdf_fullscreen(filename);
            else
                send_file(filename);

            return;
        }
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

        std::string ext = lower_ext(e.path().filename().string());
        if (allowed_ext.find(ext) == allowed_ext.end())
            continue;

        files.emplace_back(e.path().filename().string());
    }

    /* ---------- Zufällige Reihenfolge ---------- */
    std::random_device rd;
    std::mt19937 rng(rd());
    std::shuffle(files.begin(), files.end(), rng);

    /* ---------- HTML Liste ---------- */
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
        << "<title>Dokumente</title>"
        << "<style>body{font-family:system-ui;margin:3em}</style>"
        << "</head><body><h1>Dokumente</h1><ul>";

    for (const auto& f : files) {
        std::cout
            << "<li><a href='?open="
            << html_escape(f)
            << "'>"
            << html_escape(f)
            << "</a></li>";
    }

    std::cout << "</ul></body></html>";
}
