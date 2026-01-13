#pragma once
#include <string>
#include <filesystem>

void render_video_view(const std::string& file);
void render_pdf_view(const std::string& file);

// Optional: Wrapper für Fullscreen-Aufrufe aus main.cpp / dispatcher
inline void render_mp4_fullscreen(const std::string& file) { render_video_view(file); }
inline void render_pdf_fullscreen(const std::string& file) { render_pdf_view(file); }
