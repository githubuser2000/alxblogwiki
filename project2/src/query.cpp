#include "query.h"
#include <cstdlib>
#include <string>

std::string query_file() {
    const char* qs = std::getenv("QUERY_STRING");
    if (!qs) return {};

    std::string q(qs);
    auto p = q.find("file=");
    if (p == std::string::npos) return {};

    std::string v = q.substr(p + 5);
    if (v.find("..") != std::string::npos) return {};
    if (v.find('/') != std::string::npos) return {};
    return v;
}
