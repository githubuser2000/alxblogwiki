#include "FileUtils.h"
#include <filesystem>
#include <fstream>

namespace fs = std::filesystem;

namespace FileUtils {

bool isTextFile(const std::string& filepath) {
    std::ifstream file(filepath, std::ios::binary);
    if(!file) return false;
    char c;
    while(file.get(c)) {
        if(c == '\0') return false;
    }
    return true;
}

std::vector<std::string> listTextFiles(const std::string& directory, size_t max_size) {
    std::vector<std::string> files;
    for(auto& p: fs::recursive_directory_iterator(directory)) {
        if(!fs::is_regular_file(p)) continue;
        if(fs::file_size(p) > max_size) continue;
        if(isTextFile(p.path().string()))
            files.push_back(p.path().string());
    }
    return files;
}

}
