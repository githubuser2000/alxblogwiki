#ifndef RETA_TYPES_HPP
#define RETA_TYPES_HPP

#include <vector>
#include <string>
#include <map>
#include <set>
#include <memory>
#include <functional>
#include <algorithm>
#include <cmath>
#include <cstdint>

// Type aliases
template<typename T>
using Vector = std::vector<T>;

template<typename T>
using Set = std::set<T>;

template<typename K, typename V>
using Map = std::map<K, V>;

template<typename K, typename V>
using OrderedMap = std::map<K, V>; // std::map is already ordered

// In C++, std::set is ordered by default
template<typename T>
using OrderedSet = std::set<T>;

// Enums
enum class SyntaxType {
    Default,
    Nichts,
    Markdown,
    BBCode,
    Html,
    Csv,
    Emacs
};

enum class SpaltenTag {
    SternPolygon,
    Universum,
    Galaxie,
    Emotion,
    Groesse
};

// Parameter value with type erasure for lambda functions
class ParameterValue {
public:
    virtual ~ParameterValue() = default;
    virtual std::string to_string() const = 0;
    virtual std::unique_ptr<ParameterValue> clone() const = 0;
};

template<typename T>
class TypedParameterValue : public ParameterValue {
private:
    T value_;
    
public:
    explicit TypedParameterValue(const T& value) : value_(value) {}
    
    const T& get() const { return value_; }
    T& get() { return value_; }
    
    std::string to_string() const override {
        // Implement based on T
        return "ParameterValue";
    }
    
    std::unique_ptr<ParameterValue> clone() const override {
        return std::make_unique<TypedParameterValue<T>>(value_);
    }
};

// Lambda function wrapper
class LambdaParameterValue : public ParameterValue {
private:
    std::function<Set<int32_t>(const std::string&)> func_;
    
public:
    explicit LambdaParameterValue(std::function<Set<int32_t>(const std::string&)> func) 
        : func_(std::move(func)) {}
    
    Set<int32_t> operator()(const std::string& s) const {
        return func_(s);
    }
    
    std::string to_string() const override {
        return "Lambda(...)";
    }
    
    std::unique_ptr<ParameterValue> clone() const override {
        return std::make_unique<LambdaParameterValue>(func_);
    }
};

// Color configuration
struct ColorConfig {
    bool enabled = true;
    std::string even_bg = "\033[47m";
    std::string odd_bg = "\033[100m";
    std::string prime_fg = "\033[103m\033[30m\033[1m";
    std::string moon_fg = "\033[106m\033[30m";
    std::string header_bg = "\033[41m\033[30m\033[4m";
    std::string reset = "\033[0m";
};

// Table configuration
struct TableConfig {
    std::pair<int32_t, int32_t> hoechste_zeile = {1024, 163};
    int32_t text_width = 21;
    int32_t text_height = 0;
    bool nummeriere = true;
    bool keine_ueberschriften = false;
    bool spalte_gestirn = false;
    bool keine_leeren_inhalte = false;
    Vector<int32_t> breiten;
    int32_t shell_rows_amount = 24;
};

// Cell types
enum class CellType {
    Text,
    Number,
    Header,
    Generated,
    Empty
};

// Cell structure
struct Cell {
    Vector<std::string> lines;
    Map<std::string, std::string> metadata;
    CellType cell_type = CellType::Text;
    
    Cell() = default;
    
    explicit Cell(const std::string& content) {
        lines.push_back(content);
        cell_type = CellType::Text;
    }
    
    explicit Cell(const Vector<std::string>& lines_vec) 
        : lines(lines_vec), cell_type(CellType::Text) {}
    
    static Cell empty() {
        Cell cell;
        cell.lines.push_back("");
        cell.cell_type = CellType::Empty;
        return cell;
    }
    
    bool is_empty() const {
        return lines.empty() || (lines.size() == 1 && lines[0].empty());
    }
    
    std::string join_lines(const std::string& separator) const {
        std::string result;
        for (size_t i = 0; i < lines.size(); ++i) {
            if (i > 0) result += separator;
            result += lines[i];
        }
        return result;
    }
};

// Table types
using Row = Vector<Cell>;
using Table = Vector<Row>;

// Table data structure
struct TableData {
    Table table;
    Row headers;
    Vector<CellType> column_types;
    Vector<size_t> column_widths;
    
    TableData() = default;
    
    size_t num_rows() const { return table.size(); }
    size_t num_cols() const { 
        return table.empty() ? 0 : table[0].size(); 
    }
};

// Mathematical functions
inline bool is_prime(int32_t n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    
    for (int32_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

inline Vector<int32_t> prime_factors(int32_t n) {
    Vector<int32_t> factors;
    
    // Handle 2 separately
    while (n % 2 == 0) {
        factors.push_back(2);
        n /= 2;
    }
    
    // Check odd numbers
    for (int32_t i = 3; i * i <= n; i += 2) {
        while (n % i == 0) {
            factors.push_back(i);
            n /= i;
        }
    }
    
    // If n is still greater than 1, it's a prime
    if (n > 1) {
        factors.push_back(n);
    }
    
    return factors;
}

inline int32_t prim_creativity(int32_t n) {
    if (is_prime(n)) {
        // For prime numbers, creativity is based on position
        const int32_t small_primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31};
        for (int32_t prime : small_primes) {
            if (n == prime) return 1;
        }
        return 2;
    } else {
        // For composite numbers, creativity is based on prime factors
        auto factors = prime_factors(n);
        if (factors.size() == 1) {
            return 0;
        } else {
            return static_cast<int32_t>(factors.size());
        }
    }
}

inline std::pair<int32_t, Vector<int32_t>> moon_number(int32_t n) {
    Vector<int32_t> result;
    
    // Check if n is a "moon number" (has special properties)
    if (n % 4 == 0) result.push_back(4);
    if (n % 7 == 0) result.push_back(7);
    if (n % 13 == 0) result.push_back(13);
    if (n % 28 == 0) result.push_back(28);
    
    return {n, result};
}

// Range specification
struct RangeSpec {
    std::optional<int32_t> start;
    std::optional<int32_t> end;
    std::optional<int32_t> step;
    bool invert = false;
    
    static RangeSpec parse(const std::string& s) {
        RangeSpec spec;
        
        if (s.empty()) return spec;
        
        std::string str = s;
        if (str[0] == '!') {
            spec.invert = true;
            str = str.substr(1);
        }
        
        size_t dash_pos = str.find('-');
        if (dash_pos != std::string::npos) {
            std::string start_str = str.substr(0, dash_pos);
            std::string rest = str.substr(dash_pos + 1);
            
            if (!start_str.empty()) {
                spec.start = std::stoi(start_str);
            }
            
            size_t colon_pos = rest.find(':');
            if (colon_pos != std::string::npos) {
                std::string end_str = rest.substr(0, colon_pos);
                std::string step_str = rest.substr(colon_pos + 1);
                
                if (!end_str.empty()) {
                    spec.end = std::stoi(end_str);
                }
                if (!step_str.empty()) {
                    spec.step = std::stoi(step_str);
                }
            } else if (!rest.empty()) {
                spec.end = std::stoi(rest);
            }
        } else {
            // Single number
            spec.start = std::stoi(str);
            spec.end = spec.start;
        }
        
        if (!spec.step.has_value()) {
            spec.step = 1;
        }
        
        return spec;
    }
    
    Set<int32_t> to_numbers(int32_t max_value) const {
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
};

#endif // RETA_TYPES_HPP
