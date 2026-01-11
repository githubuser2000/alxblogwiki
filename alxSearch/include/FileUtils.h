#pragma once
#include <string>
#include <vector>

namespace FileUtils {
    std::vector<std::string> listTextFiles(const std::string& directory, size_t max_size);
    bool isTextFile(const std::string& filepath);
}
