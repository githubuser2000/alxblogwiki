#include "renderer.h"
#include "html.h"
#include <fstream>
#include <iostream>
#include <cstdio>

bool render_markdown_or_org(const std::filesystem::path& file) {
    std::string cmd =
        "pandoc --standalone --from=" +
        std::string(file.extension() == ".org" ? "org" : "markdown") +
        " --to=html5 \"" + file.string() + "\"";

    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe) return false;

    char buf[4096];
    while (fgets(buf, sizeof(buf), pipe))
        std::cout << buf;

    pclose(pipe);
    return true;
}

void render_text_file(const std::filesystem::path& file) {
    std::cout << "<pre>";
    std::ifstream in(file);
    std::string line;
    while (std::getline(in, line)) std::cout << html_escape(line) << "\n";
    std::cout << "</pre>";
}
