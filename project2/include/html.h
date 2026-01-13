#pragma once
#include <string>

std::string html_escape(const std::string& s);
void print_http_headers();
void render_error(const std::string& msg);
