#include "util.h"
#include "fileview.h"
#include "media_view.h"    // für render_video_view, render_pdf_view
#include "renderer.h"      // für render_error, render_text_view
#include "config.h"

#include <filesystem>
#include <string>
#include <algorithm>

namespace fs = std::filesystem;

void render_file_view(const std::string& filename) {
    fs::path path = DOCROOT / filename;

    if (!fs::exists(path) || !fs::is_regular_file(path)) {
        render_error("File not found");
        return;
    }

    std::string ext = lower_ext(filename);

    if (ext == ".mp4")
        render_video_view(filename);
    else if (ext == ".pdf")
        render_pdf_view(filename);
    else
        render_text_file(path);  // <-- hier korrigiert
}
