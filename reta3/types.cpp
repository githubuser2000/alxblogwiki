#include "types.hpp"
#include <sstream>
#include <algorithm>
#include <iomanip>

// ParameterValue implementations
std::string ParameterValue::to_string() const {
    return "ParameterValue";
}

// TypedParameterValue specializations
template<>
std::string TypedParameterValue<std::string>::to_string() const {
    return value_;
}

template<>
std::string TypedParameterValue<int>::to_string() const {
    return std::to_string(value_);
}

template<>
std::string TypedParameterValue<double>::to_string() const {
    return std::to_string(value_);
}

template<>
std::string TypedParameterValue<std::vector<std::string>>::to_string() const {
    std::stringstream ss;
    ss << "[";
    for (size_t i = 0; i < value_.size(); ++i) {
        if (i > 0) ss << ", ";
        ss << value_[i];
    }
    ss << "]";
    return ss.str();
}

// RangeSpec implementations
RangeSpec RangeSpec::parse(const std::string& s) {
    RangeSpec spec;
    
    if (s.empty()) return spec;
    
    std::string str = s;
    
    // Check for invert
    if (str[0] == '!') {
        spec.invert = true;
        str = str.substr(1);
    }
    
    // Parse the range
    size_t dash_pos = str.find('-');
    if (dash_pos != std::string::npos) {
        // Has range
        std::string start_str = str.substr(0, dash_pos);
        std::string rest = str.substr(dash_pos + 1);
        
        if (!start_str.empty()) {
            try {
                spec.start = std::stoi(start_str);
            } catch (...) {
                throw std::invalid_argument("Invalid start in range: " + start_str);
            }
        }
        
        // Check for step
        size_t colon_pos = rest.find(':');
        if (colon_pos != std::string::npos) {
            std::string end_str = rest.substr(0, colon_pos);
            std::string step_str = rest.substr(colon_pos + 1);
            
            if (!end_str.empty()) {
                try {
                    spec.end = std::stoi(end_str);
                } catch (...) {
                    throw std::invalid_argument("Invalid end in range: " + end_str);
                }
            }
            
            if (!step_str.empty()) {
                try {
                    spec.step = std::stoi(step_str);
                } catch (...) {
                    throw std::invalid_argument("Invalid step in range: " + step_str);
                }
            }
        } else if (!rest.empty()) {
            // No step, just end
            try {
                spec.end = std::stoi(rest);
            } catch (...) {
                throw std::invalid_argument("Invalid end in range: " + rest);
            }
        }
    } else {
        // Single number
        try {
            spec.start = std::stoi(str);
            spec.end = spec.start;
        } catch (...) {
            throw std::invalid_argument("Invalid number: " + str);
        }
    }
    
    // Set defaults
    if (!spec.step.has_value()) {
        spec.step = 1;
    }
    
    return spec;
}

Set<int32_t> RangeSpec::to_numbers(int32_t max_value) const {
    Set<int32_t> numbers;
    
    int32_t start_val = start.value_or(1);
    int32_t end_val = end.value_or(max_value);
    int32_t step_val = step.value_or(1);
    
    if (step_val > 0) {
        for (int32_t i = start_val; i <= end_val; i += step_val) {
            numbers.insert(i);
        }
    } else if (step_val < 0) {
        for (int32_t i = start_val; i >= end_val; i += step_val) {
            numbers.insert(i);
        }
    }
    
    if (invert) {
        Set<int32_t> all_numbers;
        for (int32_t i = 1; i <= max_value; ++i) {
            all_numbers.insert(i);
        }
        
        Set<int32_t> inverted;
        std::set_difference(
            all_numbers.begin(), all_numbers.end(),
            numbers.begin(), numbers.end(),
            std::inserter(inverted, inverted.begin())
        );
        return inverted;
    }
    
    return numbers;
}

// Cell implementations
Cell::Cell() : cell_type(CellType::Text) {}

Cell::Cell(const std::string& content) : cell_type(CellType::Text) {
    lines.push_back(content);
}

Cell::Cell(const Vector<std::string>& lines_vec) 
    : lines(lines_vec), cell_type(CellType::Text) {}

bool Cell::is_empty() const {
    return lines.empty() || 
           (lines.size() == 1 && lines[0].empty()) ||
           cell_type == CellType::Empty;
}

std::string Cell::join_lines(const std::string& separator) const {
    std::string result;
    for (size_t i = 0; i < lines.size(); ++i) {
        if (i > 0) result += separator;
        result += lines[i];
    }
    return result;
}

Cell Cell::empty() {
    Cell cell;
    cell.lines.push_back("");
    cell.cell_type = CellType::Empty;
    return cell;
}

// TableData implementations
TableData::TableData() {}

size_t TableData::num_rows() const {
    return table.size();
}

size_t TableData::num_cols() const {
    if (table.empty()) return 0;
    return table[0].size();
}

// Mathematical functions (already implemented in header)
// Additional helper functions

std::string to_string(SyntaxType type) {
    switch (type) {
        case SyntaxType::Default: return "Default";
        case SyntaxType::Nichts: return "Nichts";
        case SyntaxType::Markdown: return "Markdown";
        case SyntaxType::BBCode: return "BBCode";
        case SyntaxType::Html: return "Html";
        case SyntaxType::Csv: return "Csv";
        case SyntaxType::Emacs: return "Emacs";
        default: return "Unknown";
    }
}

std::string to_string(SpaltenTag tag) {
    switch (tag) {
        case SpaltenTag::SternPolygon: return "SternPolygon";
        case SpaltenTag::Universum: return "Universum";
        case SpaltenTag::Galaxie: return "Galaxie";
        case SpaltenTag::Emotion: return "Emotion";
        case SpaltenTag::Groesse: return "Groesse";
        default: return "Unknown";
    }
}

std::string to_string(CellType type) {
    switch (type) {
        case CellType::Text: return "Text";
        case CellType::Number: return "Number";
        case CellType::Header: return "Header";
        case CellType::Generated: return "Generated";
        case CellType::Empty: return "Empty";
        default: return "Unknown";
    }
}
