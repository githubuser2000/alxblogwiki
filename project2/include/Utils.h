#ifndef UTILS_H
#define UTILS_H
#include <string>

std::string get_query_param(const std::string& query, const std::string& key);
std::string map_to_web_path(const std::string& full_sys_path, const std::string& sys_root, const std::string& web_root);

#endif

