#pragma once
#include <string>

// für main.cpp
void render_mp4_fullscreen(const std::string& file);
void render_pdf_fullscreen(const std::string& file);

// für fileview.cpp
void render_video_view(const std::string& file);
void render_pdf_view(const std::string& file);
void render_text_view(const std::string& file);
void render_error(const std::string& msg);
