#include "config.h"
#include "html.h"
#include "query.h"
#include "renderer.h"
#include "listview.h"
#include "csv.h"
#include "media_view.h"
#include "pdf_template.h"

#include <filesystem>
#include <iostream>
#include <algorithm>

namespace fs = std::filesystem;

int main() {
    // ---- HTTP + Security Headers ----
    print_http_headers();

    // ---- Query auswerten ----
    std::string file = query_file();

    // ================= LIST VIEW =================
    if (file.empty()) {
        render_list_view();
        return 0;
    }

    // ================= FILE CHECK =================
    fs::path p = fs::path(DOCROOT) / file;
    if (!fs::exists(p) || !fs::is_regular_file(p) || fs::is_symlink(p)) {
        std::cout << "<h1>404 - File not found</h1>";
        return 0;
    }

    // Extension normalisieren
    std::string ext = p.extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(),
                   [](unsigned char c){ return std::tolower(c); });

    // ================= MEDIA (FULLSCREEN) =================
    if (ext == ".mp4") {
        render_mp4_fullscreen(file);  // Wrapper für render_video_view
        return 0;
    }

    if (ext == ".pdf") {
        render_pdf_template_redirect(file);  // PDF über HTML Template + JS öffnen
        return 0;
    }

    // ================= HTML WRAPPER =================
    std::cout << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
              << "<title>" << html_escape(file) << "</title>"
              << "<style>body{margin:0;padding:4em;font-family:serif}"
              << "a{position:fixed;top:1em;left:1em}</style></head><body>"
              << "<a href='?'>← zurück</a>";

    // ================= DISPATCH =================
    if (ext == ".md" || ext == ".org") {
        if (!render_markdown_or_org(p))
            render_text_file(p);
    }
    else if (ext == ".csv") {
        render_csv_fullscreen(p);
    }
    else if (ext == ".txt" || ext == ".tx") {
        render_text_file(p);
    }
    else {
        std::cout << "<h1>Invalid file type</h1>";
    }

    std::cout << "</body></html>";
    return 0;
}
