#ifndef RETA_ERROR_HPP
#define RETA_ERROR_HPP

#include <exception>
#include <string>
#include <sstream>

class RETAException : public std::exception {
private:
    std::string message_;
    
public:
    explicit RETAException(const std::string& message) : message_(message) {}
    
    const char* what() const noexcept override {
        return message_.c_str();
    }
    
    static RETAException io_error(const std::string& message) {
        return RETAException("I/O error: " + message);
    }
    
    static RETAException csv_error(const std::string& message) {
        return RETAException("CSV error: " + message);
    }
    
    static RETAException parse_error(const std::string& message) {
        return RETAException("Parse error: " + message);
    }
    
    static RETAException invalid_parameter(const std::string& message) {
        return RETAException("Invalid parameter: " + message);
    }
    
    static RETAException file_not_found(const std::string& message) {
        return RETAException("File not found: " + message);
    }
    
    static RETAException config_error(const std::string& message) {
        return RETAException("Configuration error: " + message);
    }
    
    static RETAException table_error(const std::string& message) {
        return RETAException("Table error: " + message);
    }
    
    static RETAException unknown_command(const std::string& message) {
        return RETAException("Unknown command: " + message);
    }
};

#endif // RETA_ERROR_HPP
