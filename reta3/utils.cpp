#include "utils.hpp"
#include <cstdlib>
#include <algorithm>
#include <cctype>
#include <locale>

namespace utils {

// String trimming utilities
static inline std::string ltrim(const std::string& s) {
    size_t start = s.find_first_not_of(" \t\n\r\f\v");
    return (start == std::string::npos) ? "" : s.substr(start);
}

static inline std::string rtrim(const std::string& s) {
    size_t end = s.find_last_not_of(" \t\n\r\f\v");
    return (end == std::string::npos) ? "" : s.substr(0, end + 1);
}

std::string trim(const std::string& s) {
    return rtrim(ltrim(s));
}

// Split string by delimiter
std::vector<std::string> split(const std::string& s, char delimiter) {
    std::vector<std::string> tokens;
    std::string token;
    std::istringstream token_stream(s);
    
    while (std::getline(token_stream, token, delimiter)) {
        tokens.push_back(trim(token));
    }
    
    return tokens;
}

// Check if string starts with prefix
bool starts_with(const std::string& s, const std::string& prefix) {
    return s.size() >= prefix.size() && 
           s.compare(0, prefix.size(), prefix) == 0;
}

// Check if string ends with suffix
bool ends_with(const std::string& s, const std::string& suffix) {
    return s.size() >= suffix.size() && 
           s.compare(s.size() - suffix.size(), suffix.size(), suffix) == 0;
}

// Join vector of strings with delimiter
std::string join(const std::vector<std::string>& parts, const std::string& delimiter) {
    std::string result;
    for (size_t i = 0; i < parts.size(); ++i) {
        if (i > 0) result += delimiter;
        result += parts[i];
    }
    return result;
}

// Get terminal dimensions
std::tuple<int32_t, int32_t, int32_t, int32_t> get_text_wrap_things() {
    int32_t rows = 24;
    int32_t cols = 80;
    
#ifdef _WIN32
    // Windows-specific terminal size detection
    #ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
    #define ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004
    #endif
    
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    if (hConsole != INVALID_HANDLE_VALUE) {
        CONSOLE_SCREEN_BUFFER_INFO csbi;
        if (GetConsoleScreenBufferInfo(hConsole, &csbi)) {
            cols = csbi.srWindow.Right - csbi.srWindow.Left + 1;
            rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
        }
    }
#else
    // Unix/Linux/MacOS terminal size detection
    struct winsize w;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == 0) {
        rows = w.ws_row;
        cols = w.ws_col;
    } else {
        // Try environment variables
        char* rows_env = std::getenv("ROWS");
        char* cols_env = std::getenv("COLUMNS");
        
        if (rows_env) {
            try { rows = std::stoi(rows_env); } catch (...) {}
        }
        
        if (cols_env) {
            try { cols = std::stoi(cols_env); } catch (...) {}
        }
        
        // Try LINES and COLUMNS (common on Unix)
        if (rows_env == nullptr) {
            rows_env = std::getenv("LINES");
            if (rows_env) {
                try { rows = std::stoi(rows_env); } catch (...) {}
            }
        }
    }
#endif
    
    return {rows, cols, 0, 0};
}

// Set shell rows amount
void set_shell_rows_amount(int32_t amount) {
    std::string amount_str = std::to_string(amount);
#ifdef _WIN32
    _putenv_s("ROWS", amount_str.c_str());
#else
    setenv("ROWS", amount_str.c_str(), 1);
#endif
}

// Get shell rows amount
int32_t shell_rows_amount() {
    auto [rows, cols, _, __] = get_text_wrap_things();
    return rows;
}

// Parse range string to numbers
std::vector<int32_t> parse_range(const std::string& s) {
    std::vector<int32_t> numbers;
    
    if (s.empty()) return numbers;
    
    std::stringstream ss(s);
    std::string part;
    
    while (std::getline(ss, part, ',')) {
        part = trim(part);
        if (part.empty()) continue;
        
        // Check for range
        size_t dash_pos = part.find('-');
        if (dash_pos != std::string::npos) {
            std::string start_str = part.substr(0, dash_pos);
            std::string end_str = part.substr(dash_pos + 1);
            
            start_str = trim(start_str);
            end_str = trim(end_str);
            
            try {
                int32_t start = start_str.empty() ? 1 : std::stoi(start_str);
                int32_t end = end_str.empty() ? 1000 : std::stoi(end_str);
                
                if (start <= end) {
                    for (int32_t i = start; i <= end; ++i) {
                        numbers.push_back(i);
                    }
                } else {
                    for (int32_t i = start; i >= end; --i) {
                        numbers.push_back(i);
                    }
                }
            } catch (const std::exception& e) {
                throw RETAException("Invalid range '" + part + "': " + e.what());
            }
        } else {
            // Single number
            try {
                numbers.push_back(std::stoi(part));
            } catch (const std::exception& e) {
                throw RETAException("Invalid number '" + part + "': " + e.what());
            }
        }
    }
    
    // Remove duplicates and sort
    std::sort(numbers.begin(), numbers.end());
    numbers.erase(std::unique(numbers.begin(), numbers.end()), numbers.end());
    
    return numbers;
}

// HTML escaping
std::string html_escape(const std::string& s) {
    std::string result;
    result.reserve(s.length() * 2);
    
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

// Parse numbers from a range specification (comma and dash separated)
std::vector<int32_t> bereich_to_numbers(const std::string& s) {
    std::vector<int32_t> result;
    
    if (s.empty()) return result;
    
    std::stringstream ss(s);
    std::string part;
    
    while (std::getline(ss, part, ',')) {
        part = trim(part);
        if (part.empty()) continue;
        
        // Check for negation
        bool negate = false;
        if (part[0] == '!') {
            negate = true;
            part = part.substr(1);
            part = trim(part);
        }
        
        size_t dash_pos = part.find('-');
        if (dash_pos != std::string::npos) {
            std::string start_str = part.substr(0, dash_pos);
            std::string end_str = part.substr(dash_pos + 1);
            
            try {
                int32_t start = std::stoi(start_str);
                int32_t end = std::stoi(end_str);
                
                std::vector<int32_t> range_numbers;
                if (start <= end) {
                    for (int32_t i = start; i <= end; ++i) {
                        range_numbers.push_back(i);
                    }
                } else {
                    for (int32_t i = start; i >= end; --i) {
                        range_numbers.push_back(i);
                    }
                }
                
                if (negate) {
                    // For now, we don't handle negation properly in this simplified version
                    // In a full implementation, we would need the complete set to negate against
                    result.insert(result.end(), range_numbers.begin(), range_numbers.end());
                } else {
                    result.insert(result.end(), range_numbers.begin(), range_numbers.end());
                }
            } catch (...) {
                // Ignore invalid ranges
            }
        } else {
            try {
                int32_t num = std::stoi(part);
                result.push_back(num);
            } catch (...) {
                // Ignore invalid numbers
            }
        }
    }
    
    // Remove duplicates and sort
    std::sort(result.begin(), result.end());
    result.erase(std::unique(result.begin(), result.end()), result.end());
    
    return result;
}

// Format table with consistent column widths
std::string format_table(const std::vector<std::vector<std::string>>& rows, 
                        const std::string& separator) {
    if (rows.empty()) return "";
    
    // Find maximum width for each column
    size_t num_cols = 0;
    for (const auto& row : rows) {
        num_cols = std::max(num_cols, row.size());
    }
    
    std::vector<size_t> col_widths(num_cols, 0);
    for (const auto& row : rows) {
        for (size_t col_idx = 0; col_idx < row.size(); ++col_idx) {
            col_widths[col_idx] = std::max(col_widths[col_idx], row[col_idx].size());
        }
    }
    
    // Format each row
    std::string result;
    for (const auto& row : rows) {
        for (size_t col_idx = 0; col_idx < row.size(); ++col_idx) {
            if (col_idx > 0) result += separator;
            
            const auto& cell = row[col_idx];
            size_t width = col_widths[col_idx];
            
            // Add padding
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

// Convert string to lowercase
std::string to_lower(const std::string& s) {
    std::string result = s;
    std::transform(result.begin(), result.end(), result.begin(),
                   [](unsigned char c) { return std::tolower(c); });
    return result;
}

// Convert string to uppercase
std::string to_upper(const std::string& s) {
    std::string result = s;
    std::transform(result.begin(), result.end(), result.begin(),
                   [](unsigned char c) { return std::toupper(c); });
    return result;
}

// Check if string contains only digits
bool is_digits(const std::string& s) {
    return !s.empty() && std::all_of(s.begin(), s.end(), ::isdigit);
}

// Replace all occurrences in a string
std::string replace_all(const std::string& s, const std::string& from, 
                       const std::string& to) {
    std::string result = s;
    size_t start_pos = 0;
    
    while ((start_pos = result.find(from, start_pos)) != std::string::npos) {
        result.replace(start_pos, from.length(), to);
        start_pos += to.length();
    }
    
    return result;
}

// Read entire file into string
std::string read_file(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    if (!file) {
        throw RETAException::file_not_found("Cannot open file: " + filename);
    }
    
    std::string content;
    file.seekg(0, std::ios::end);
    content.reserve(file.tellg());
    file.seekg(0, std::ios::beg);
    
    content.assign(std::istreambuf_iterator<char>(file),
                   std::istreambuf_iterator<char>());
    
    return content;
}

// Write string to file
void write_file(const std::string& filename, const std::string& content) {
    std::ofstream file(filename, std::ios::binary);
    if (!file) {
        throw RETAException::io_error("Cannot write to file: " + filename);
    }
    
    file.write(content.c_str(), content.size());
}

// Parse CSV line
std::vector<std::string> parse_csv_line(const std::string& line, char delimiter) {
    std::vector<std::string> fields;
    std::string field;
    bool in_quotes = false;
    
    for (size_t i = 0; i < line.length(); ++i) {
        char c = line[i];
        
        if (c == '"') {
            // Handle escaped quotes
            if (i + 1 < line.length() && line[i + 1] == '"') {
                field += '"';
                ++i;
            } else {
                in_quotes = !in_quotes;
            }
        } else if (c == delimiter && !in_quotes) {
            fields.push_back(trim(field));
            field.clear();
        } else {
            field += c;
        }
    }
    
    fields.push_back(trim(field));
    return fields;
}

// Format CSV line
std::string format_csv_line(const std::vector<std::string>& fields, char delimiter) {
    std::string line;
    
    for (size_t i = 0; i < fields.size(); ++i) {
        if (i > 0) line += delimiter;
        
        const std::string& field = fields[i];
        
        // Check if field needs quoting
        bool needs_quotes = field.find(delimiter) != std::string::npos ||
                           field.find('"') != std::string::npos ||
                           field.find('\n') != std::string::npos ||
                           field.find('\r') != std::string::npos;
        
        if (needs_quotes) {
            line += '"';
            line += replace_all(field, "\"", "\"\"");
            line += '"';
        } else {
            line += field;
        }
    }
    
    return line;
}

// Parse JSON (simplified implementation)
std::map<std::string, std::string> parse_simple_json(const std::string& json_str) {
    std::map<std::string, std::string> result;
    
    // Remove whitespace
    std::string json = trim(json_str);
    
    // Check if it's an object
    if (json.front() != '{' || json.back() != '}') {
        throw RETAException::parse_error("Invalid JSON object");
    }
    
    // Remove braces
    json = json.substr(1, json.length() - 2);
    
    // Parse key-value pairs
    size_t pos = 0;
    while (pos < json.length()) {
        // Find key
        size_t key_start = json.find('"', pos);
        if (key_start == std::string::npos) break;
        
        size_t key_end = json.find('"', key_start + 1);
        if (key_end == std::string::npos) break;
        
        std::string key = json.substr(key_start + 1, key_end - key_start - 1);
        
        // Find colon
        size_t colon_pos = json.find(':', key_end + 1);
        if (colon_pos == std::string::npos) break;
        
        // Find value
        size_t value_start = json.find_first_not_of(" \t\n\r", colon_pos + 1);
        if (value_start == std::string::npos) break;
        
        std::string value;
        if (json[value_start] == '"') {
            // String value
            size_t value_end = json.find('"', value_start + 1);
            if (value_end == std::string::npos) break;
            
            value = json.substr(value_start + 1, value_end - value_start - 1);
            pos = value_end + 1;
        } else {
            // Other value (simplified)
            size_t value_end = json.find_first_of(",}", value_start);
            if (value_end == std::string::npos) break;
            
            value = trim(json.substr(value_start, value_end - value_start));
            pos = value_end;
        }
        
        result[key] = value;
        
        // Skip comma
        if (pos < json.length() && json[pos] == ',') {
            ++pos;
        }
    }
    
    return result;
}

} // namespace utils
