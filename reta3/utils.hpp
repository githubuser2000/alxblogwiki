#ifndef RETA_UTILS_HPP
#define RETA_UTILS_HPP

#include "types.hpp"
#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cstdlib>
#include <cstring>

namespace utils {

inline std::tuple<int32_t, int32_t, int32_t, int32_t> get_text_wrap_things() {
    int32_t rows = 24;
    int32_t cols = 80;
    
    // Try to get from environment
    const char* rows_env = std::getenv("ROWS");
    const char* cols_env = std::getenv("COLUMNS");
    
    if (rows_env) {
        try { rows = std::stoi(rows_env); } catch (...) {}
    }
    
    if (cols_env) {
        try { cols = std::stoi(cols_env); } catch (...) {}
    }
    
    return {rows, cols, 0, 0};
}

inline void set_shell_rows_amount(int32_t amount) {
    std::string amount_str = std::to_string(amount);
#ifdef _WIN32
    _putenv_s("ROWS", amount_str.c_str());
#else
    setenv("ROWS", amount_str.c_str(), 1);
#endif
}

inline int32_t shell_rows_amount() {
    auto [rows, cols, _, __] = get_text_wrap_things();
    return rows;
}

inline Vector<int32_t> bereich_to_numbers(const std::string& s) {
    Vector<int32_t> result;
    std::stringstream ss(s);
    std::string part;
    
    while (std::getline(ss, part, ',')) {
        // Trim whitespace
        part.erase(0, part.find_first_not_of(" \t\n\r"));
        part.erase(part.find_last_not_of(" \t\n\r") + 1);
        
        if (part.empty()) continue;
        
        size_t dash_pos = part.find('-');
        if (dash_pos != std::string::npos) {
            std::string start_str = part.substr(0, dash_pos);
            std::string end_str = part.substr(dash_pos + 1);
            
            try {
                int32_t start = std::stoi(start_str);
                int32_t end = std::stoi(end_str);
                
                if (start <= end) {
                    for (int32_t i = start; i <= end; ++i) {
                        result.push_back(i);
                    }
                } else {
                    for (int32_t i = start; i >= end; --i) {
                        result.push_back(i);
                    }
                }
            } catch (...) {
                // Ignore invalid numbers
            }
        } else {
            try {
                result.push_back(std::stoi(part));
            } catch (...) {
                // Ignore invalid numbers
            }
        }
    }
    
    return result;
}

inline std::string html_escape(const std::string& s) {
    std::string result;
    result.reserve(s.size() * 2);
    
    for (char c : s) {
        switch (c) {
            case '&':  result += "&amp;"; break;
            case '<':  result += "&lt;"; break;
            case '>':  result += "&gt;"; break;
            case '"':  result += "&quot;"; break;
            case '\'': result += "&#39;"; break;
            default:   result += c; break;
        }
    }
    
    return result;
}

inline Vector<int32_t> parse_range(const std::string& s) {
    Vector<int32_t> numbers;
    std::stringstream ss(s);
    std::string part;
    
    while (std::getline(ss, part, ',')) {
        // Trim
        part.erase(0, part.find_first_not_of(" \t\n\r"));
        part.erase(part.find_last_not_of(" \t\n\r") + 1);
        
        if (part.empty()) continue;
        
        size_t dash_pos = part.find('-');
        if (dash_pos != std::string::npos) {
            std::string start_str = part.substr(0, dash_pos);
            std::string end_str = part.substr(dash_pos + 1);
            
            try {
                int32_t start = std::stoi(start_str);
                int32_t end = std::stoi(end_str);
                
                if (start <= end) {
                    for (int32_t i = start; i <= end; ++i) {
                        numbers.push_back(i);
                    }
                } else {
                    for (int32_t i = end; i <= start; ++i) {
                        numbers.push_back(i);
                    }
                }
            } catch (const std::exception& e) {
                throw RETAException("Invalid range '" + part + "': " + e.what());
            }
        } else {
            try {
                numbers.push_back(std::stoi(part));
            } catch (const std::exception& e) {
                throw RETAException("Invalid number '" + part + "': " + e.what());
            }
        }
    }
    
    return numbers;
}

inline std::string format_table(const Vector<Vector<std::string>>& rows, 
                               const std::string& separator) {
    if (rows.empty()) return "";
    
    // Find maximum width for each column
    size_t num_cols = rows[0].size();
    Vector<size_t> col_widths(num_cols, 0);
    
    for (const auto& row : rows) {
        for (size_t col_idx = 0; col_idx < std::min(row.size(), num_cols); ++col_idx) {
            col_widths[col_idx] = std::max(col_widths[col_idx], row[col_idx].size());
        }
    }
    
    // Format each row
    std::string result;
    for (const auto& row : rows) {
        for (size_t col_idx = 0; col_idx < std::min(row.size(), num_cols); ++col_idx) {
            if (col_idx > 0) result += separator;
            
            const auto& cell = row[col_idx];
            size_t width = col_widths[col_idx];
            
            // Right-align numbers, left-align text
            if (cell.size() < width) {
                result += cell + std::string(width - cell.size(), ' ');
            } else {
                result += cell;
            }
        }
        result += "\n";
    }
    
    return result;
}

// String utilities
inline std::string trim(const std::string& s) {
    auto start = s.find_first_not_of(" \t\n\r");
    auto end = s.find_last_not_of(" \t\n\r");
    
    if (start == std::string::npos) return "";
    return s.substr(start, end - start + 1);
}

inline Vector<std::string> split(const std::string& s, char delimiter) {
    Vector<std::string> tokens;
    std::stringstream ss(s);
    std::string token;
    
    while (std::getline(ss, token, delimiter)) {
        tokens.push_back(trim(token));
    }
    
    return tokens;
}

inline bool starts_with(const std::string& s, const std::string& prefix) {
    return s.size() >= prefix.size() && 
           s.compare(0, prefix.size(), prefix) == 0;
}

inline bool ends_with(const std::string& s, const std::string& suffix) {
    return s.size() >= suffix.size() && 
           s.compare(s.size() - suffix.size(), suffix.size(), suffix) == 0;
}

inline std::string join(const Vector<std::string>& parts, const std::string& delimiter) {
    std::string result;
    for (size_t i = 0; i < parts.size(); ++i) {
        if (i > 0) result += delimiter;
        result += parts[i];
    }
    return result;
}

} // namespace utils

#endif // RETA_UTILS_HPP
