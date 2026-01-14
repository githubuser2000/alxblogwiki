#include "HtmlRenderer.h"
#include <iostream>

void print_http_header() {
    std::cout << "Content-Type: text/html; charset=utf-8\n\n";
}

void print_html_start(const std::string& title) {
    std::cout << "<html><head><style>"
              << "body{font-family:sans-serif; padding:20px;}"
              << ".dir{color:blue; font-weight:bold;}"
              << ".file{color:green;}"
              << "</style></head><body>"
              << "<h1>Verzeichnis: " << title << "</h1><hr><ul>";
}

void print_list_item(const std::string& label, const std::string& link, bool is_dir, const std::string& ext) {
    std::cout << "<li>";
    if (is_dir) {
        std::cout << "<span class='dir'>[DIR]</span> <a href=\"?path=" << link << "\">" << label << "</a>";
    } else {
        std::cout << "<span class='file'>[" << ext << "]</span> <a href=\"" << link << "\">" << label << "</a>";
    }
    std::cout << "</li>";
}

void print_html_end() {
    std::cout << "</ul><hr><small>C++ CGI Explorer</small></body></html>";
}

void print_list_item(const std::string& label, const std::string& link, bool is_dir, const std::string& ext, bool is_renderable) {
    std::cout << "<li>";
    if (is_dir) {
        std::cout << "<span class='dir'>[DIR]</span> <a href=\"?path=" << link << "\">" << label << "</a>";
    } else {
        std::string css_class = is_renderable ? "renderable" : "file";
        std::cout << "<span class='" << css_class << "'>[" << ext << "]</span> <a href=\"" << link << "\">" << label << "</a>";
    }
    std::cout << "</li>";
}

