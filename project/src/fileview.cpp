#include "fileview.h"
#include "media_view.h"
#include "config.h"

#include <filesystem>
#include <string>
#include <algorithm>

namespace fs = std::filesystem; 

void render_file_view(const std::string& filename)
{
    fs::path path = fs::path(DOCROOT) / filename;
    if (!fs::exists(path) || !fs::is_regular_file(path)) {
        render_error("File not found");
        return;
    }

    std::string ext = path.extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(),
                   [](unsigned char c){ return std::tolower(c); });

    if (ext == ".mp4")
        render_video_view(filename);
    else if (ext == ".pdf")
        render_pdf_view(filename);
    else
        render_text_view(filename);
}
