#include "pdf_template.h"
#include "html.h"
#include <iostream>

void render_pdf_template_redirect(const std::string& file) {
    // Weiterleitung auf HTML-Template mit JavaScript
    std::cout << "<!DOCTYPE html><html>"
              << "<head><meta charset='utf-8'><title>PDF Viewer</title></head>"
              << "<body>"
              << "<script>"
              << "window.location.href = '/docs/pdf_template.html?file=" 
              << file << "';"
              << "</script>"
              << "</body></html>";
}
