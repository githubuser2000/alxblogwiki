#ifndef HTML_RENDERER_H
#define HTML_RENDERER_H
#include <string>

void print_http_header();
void print_html_start(const std::string& title);
void print_html_end();
void print_list_item(const std::string& label, const std::string& link, bool is_dir, const std::string& ext = "", bool is_renderable = false);

#endif

