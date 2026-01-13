#include "media_view.h"
#include "html.h"
#include <iostream>

void render_video_view(const std::string& file) {
    std::cout << "Content-Type: text/html; charset=utf-8\r\n\r\n"
              << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
              << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
              << "<title>Video</title>"
              << "<style>html,body{margin:0;width:100%;height:100%;background:black;overflow:hidden} video{width:100%;height:100%;object-fit:contain}</style>"
              << "</head><body>"
              << "<video controls autoplay muted playsinline>"
              << "<source src='?raw=" << html_escape(file) << "' type='video/mp4'>"
              << "Ihr Browser unterstützt kein HTML5-Video."
              << "</video>"
              << "</body></html>";
}

void render_pdf_view(const std::string& file) {
    std::cout << "Content-Type: text/html; charset=utf-8\r\n\r\n"
              << "<!DOCTYPE html><html><head><meta charset='utf-8'>"
              << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
              << "<title>PDF</title>"
              << "<style>html,body{margin:0;width:100%;height:100%;overflow:hidden;background:#111} iframe{width:100%;height:100%;border:0}</style>"
              << "</head><body>"
              << "<iframe src='?raw=" << html_escape(file) << "'></iframe>"
              << "</body></html>";
}
