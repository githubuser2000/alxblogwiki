#pragma once
#include <string>
#include <filesystem>
#include <algorithm>

inline std::string lower_ext(const std::string& filename) {
    std::string ext = std::filesystem::path(filename).extension().string();
    std::transform(ext.begin(), ext.end(), ext.begin(),
                   [](unsigned char c){ return std::tolower(c); });
    return ext;
}
