#pragma once
#include <filesystem>

bool render_markdown_or_org(const std::filesystem::path& file);
void render_text_file(const std::filesystem::path& file);
