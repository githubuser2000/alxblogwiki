#pragma once
#include <string>
#include <filesystem>

void render_error(const std::string& msg);
void render_text_file(const std::filesystem::path& file);
bool render_markdown_or_org(const std::filesystem::path& file);
