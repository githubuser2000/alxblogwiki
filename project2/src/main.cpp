#include <iostream>
#include <filesystem>
#include "Config.h"
#include "Utils.h"
#include "HtmlRenderer.h"

namespace fs = std::filesystem;

int main() {
    char* query_raw = getenv("QUERY_STRING");
    std::string query = query_raw ? query_raw : "";
    std::string req_path = get_query_param(query, "path");

    // Validierung & Sicherheit
    if (req_path.empty() || req_path.find("..") != std::string::npos || req_path.find(Config::SYS_ROOT) != 0) {
        req_path = Config::SYS_ROOT;
    }

    fs::path current_path(req_path);
    
    print_http_header();
    print_html_start(req_path);

    try {
        // Zurück-Link
        if (!fs::equivalent(current_path, Config::SYS_ROOT)) {
		print_list_item(".. (Zurück)", current_path.parent_path().string(), true, "");
        }

           for (const auto& entry : fs::directory_iterator(current_path)) {
        std::string name = entry.path().filename().string();
        std::string full_sys_path = entry.path().string();
        std::string ext = entry.path().extension().string();

        if (entry.is_directory()) {
            print_list_item(name, full_sys_path, true);
        } else {
            std::string web_url = map_to_web_path(full_sys_path, Config::SYS_ROOT, Config::WEB_ROOT);
            // Prüfung auf renderbare Formate
            bool is_renderable = (ext == ".html" || ext == ".htm" || ext == ".md" || ext == ".org" || ext == ".txt");
            print_list_item(name, web_url, false, ext, is_renderable);
        }
    }
 
    } catch (const std::exception& e) {
        std::cout << "<p>Fehler: " << e.what() << "</p>";
    }

    print_html_end();
    return 0;
}

