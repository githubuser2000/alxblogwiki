#pragma once
#include <string>

void render_video_view(const std::string& file);
void render_pdf_view(const std::string& file);
void render_text_view(const std::string& file);
void render_error(const std::string& msg);
