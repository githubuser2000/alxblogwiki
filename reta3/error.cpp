#include "error.hpp"
#include <string>

RETAException::RETAException(const std::string& message) : message_(message) {}

const char* RETAException::what() const noexcept {
    return message_.c_str();
}

RETAException RETAException::io_error(const std::string& message) {
    return RETAException("I/O error: " + message);
}

RETAException RETAException::csv_error(const std::string& message) {
    return RETAException("CSV error: " + message);
}

RETAException RETAException::parse_error(const std::string& message) {
    return RETAException("Parse error: " + message);
}

RETAException RETAException::invalid_parameter(const std::string& message) {
    return RETAException("Invalid parameter: " + message);
}

RETAException RETAException::file_not_found(const std::string& message) {
    return RETAException("File not found: " + message);
}

RETAException RETAException::config_error(const std::string& message) {
    return RETAException("Configuration error: " + message);
}

RETAException RETAException::table_error(const std::string& message) {
    return RETAException("Table error: " + message);
}

RETAException RETAException::unknown_command(const std::string& message) {
    return RETAException("Unknown command: " + message);
}
