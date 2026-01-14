#include <iostream>
#include <string>
#include <filesystem>

namespace fs = std::filesystem;

std::string get_query_param(std::string query, std::string key) {
    size_t pos = query.find(key + "=");
    if (pos == std::string::npos) return "";
    size_t start = pos + key.length() + 1;
    size_t end = query.find("&", start);
    return query.substr(start, end - start);
}

int main() {
    // KONFIGURATION
    const std::string sys_root = "/var/www/docs"; // Pfad auf der Festplatte
    const std::string web_root = "/docs";         // Pfad in der URL (Anpassung je nach Server!)

    char* query_raw = getenv("QUERY_STRING");
    std::string query = query_raw ? query_raw : "";
    std::string req_path = get_query_param(query, "path");

    // Falls leer oder unsicher, auf sys_root setzen
    if (req_path.empty() || req_path.find("..") != std::string::npos || req_path.find(sys_root) != 0) {
        req_path = sys_root;
    }

    fs::path current_path(req_path);

    // Header ausgeben
    std::cout << "Content-Type: text/html; charset=utf-8\n\n";
    std::cout << "<html><head><style>body{font-family:sans-serif; padding:20px;} .dir{color:blue;} .file{color:green;}</style></head><body>";
    std::cout << "<h1>Index von " << req_path << "</h1><hr><ul>";

    // "Zurück"-Link
    if (!fs::equivalent(current_path, sys_root)) {
        std::cout << "<li><a href=\"?path=" << current_path.parent_path().string() << "\">.. (Zurück)</a></li>";
    }

    for (const auto& entry : fs::directory_iterator(current_path)) {
        std::string name = entry.path().filename().string();
        std::string full_sys_path = entry.path().string();
        
        // Umrechnung in Web-Pfad: Ersetze sys_root durch web_root
        std::string relative_part = full_sys_path.substr(sys_root.length());
        std::string url_path = web_root + relative_part;

        if (entry.is_directory()) {
            // Ordner: Link führt wieder zum CGI mit dem Systempfad
            std::cout << "<li><b>[DIR]</b> <a href=\"?path=" << full_sys_path << "\">" << name << "</a></li>";
        } else {
            std::string ext = entry.path().extension().string();
            // Datei: Link führt direkt zur Web-URL
            std::cout << "<li><span class='file'>[" << ext << "]</span> <a href=\"" << url_path << "\">" << name << "</a></li>";
        }
    }

    std::cout << "</ul></body></html>";
    return 0;
}

