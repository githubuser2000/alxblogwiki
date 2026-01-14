#include "Utils.h"

std::string get_query_param(const std::string& query, const std::string& key) {
    size_t pos = query.find(key + "=");
    if (pos == std::string::npos) return "";
    size_t start = pos + key.length() + 1;
    size_t end = query.find("&", start);
    return query.substr(start, end - start);
}

std::string map_to_web_path(const std::string& full_sys_path, const std::string& sys_root, const std::string& web_root) {
    if (full_sys_path.length() < sys_root.length()) return web_root;
    return web_root + full_sys_path.substr(sys_root.length());
}

