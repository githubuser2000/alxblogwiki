#include "program.hpp"
#include <algorithm>
#include <fstream>
#include <sstream>
#include <iostream>
#include <cstdlib>
#include <optional>

using namespace std;

// Helper function to check if string starts with prefix
static bool starts_with(const string& s, const string& prefix) {
    return s.size() >= prefix.size() && 
           s.compare(0, prefix.size(), prefix) == 0;
}

// Helper function to split string
static vector<string> split(const string& s, char delimiter) {
    vector<string> tokens;
    string token;
    istringstream token_stream(s);
    
    while (getline(token_stream, token, delimiter)) {
        tokens.push_back(token);
    }
    
    return tokens;
}

// Helper function to trim whitespace
static string trim(const string& s) {
    auto start = s.find_first_not_of(" \t\n\r");
    auto end = s.find_last_not_of(" \t\n\r");
    
    if (start == string::npos) return "";
    return s.substr(start, end - start + 1);
}

Program::Program(const vector<string>& argv, const string& txt, bool run_alles)
    : tables_(nullopt, txt), run_alles_(run_alles) {
    
    // Initialize argv
    argv_.reserve(argv.size());
    for (const auto& arg : argv) {
        argv_.push_back(trim(arg));
    }
    
    // Initialize spalten type naming
    spalten_type_naming_ = {
        {0, 0}, {0, 1}, {0, 2}, {0, 3}, {0, 4}, {0, 5}, 
        {0, 6}, {0, 7}, {0, 8}, {0, 9}, {0, 10}, {0, 11},
        {1, 0}, {1, 1}, {1, 2}, {1, 3}, {1, 4}, {1, 5},
        {1, 6}, {1, 7}, {1, 8}, {1, 9}, {1, 10}, {1, 11}
    };
    
    // Initialize spalten_arten map
    for (int i = 0; i < 2; ++i) {
        for (int j = 0; j < 12; ++j) {
            spalten_arten_key_spaltennummern_value_[{i, j}] = OrderedSet<int32_t>();
        }
    }
    
    if (run_alles) {
        resulting_table_ = workflow_everything(argv_);
    }
}

void Program::invert_alles() {
    invert_alles_ = true;
}

void Program::run() {
    if (!run_alles_) {
        resulting_table_ = workflow_everything(argv_);
    }
}

vector<string> Program::workflow_everything(const vector<string>& argv) {
    // Simplified implementation
    return {
        "RETA Table Output",
        "================",
        "",
        "Column1 | Column2 | Column3",
        "------- | ------- | -------",
        "Data1   | Data2   | Data3",
