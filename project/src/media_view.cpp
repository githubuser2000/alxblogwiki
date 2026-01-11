#include "media_view.h"
#include "html.h"

#include <iostream>
#include <string>

/* ------------------------------------------------------------
   MP4 Vollbild-View
------------------------------------------------------------ */
void render_video_view(const std::string& file)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html>"
        << "<html lang='de'>"
        << "<head>"
        << "<meta charset='utf-8'>"
        << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
        << "<title>Video</title>"
        << "<style>"
        << "html,body{margin:0;width:100%;height:100%;background:black;overflow:hidden}"
        << "video{width:100%;height:100%;object-fit:contain}"
        << "</style>"
        << "</head>"
        << "<body>"
        << "<video controls autoplay muted playsinline>"
        << "<source src='?raw=" << html_escape(file) << "' type='video/mp4'>"
        << "Ihr Browser unterstützt kein HTML5-Video."
        << "</video>"
        << "</body>"
        << "</html>";
}

/* ------------------------------------------------------------
   PDF Vollbild-View
------------------------------------------------------------ */
void render_pdf_view(const std::string& file)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<!DOCTYPE html>"
        << "<html lang='de'>"
        << "<head>"
        << "<meta charset='utf-8'>"
        << "<meta name='viewport' content='width=device-width,height=device-height,initial-scale=1.0'>"
        << "<title>PDF</title>"
        << "<style>"
        << "html,body{margin:0;width:100%;height:100%;overflow:hidden;background:#111}"
        << "iframe{width:100%;height:100%;border:0}"
        << "</style>"
        << "</head>"
        << "<body>"
        << "<iframe src='?raw=" << html_escape(file) << "'></iframe>"
        << "</body>"
        << "</html>";
}

/* ------------------------------------------------------------
   Fallback (nicht mehr benutzt, bleibt sauber)
------------------------------------------------------------ */
void render_text_view(const std::string& file)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<pre>Dateiformat nicht unterstützt: "
        << html_escape(file)
        << "</pre>";
}

void render_error(const std::string& msg)
{
    std::cout
        << "Content-Type: text/html; charset=utf-8\r\n\r\n"
        << "<h1>Error</h1><p>"
        << html_escape(msg)
        << "</p>";
}

/* ------------------------------------------------------------
   Öffentliche Wrapper (für main / dispatcher)
------------------------------------------------------------ */
void render_mp4_fullscreen(const std::string& file)
{
    render_video_view(file);
}

void render_pdf_fullscreen(const std::string& file)
{
    render_pdf_view(file);
}
