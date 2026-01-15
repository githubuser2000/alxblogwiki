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
        "Data4   | Data5   | Data6"
    };
}

void Program::produce_all_spalten_numbers(const string& neg) {
    // Lambda functions
    auto lambda_gebr_univ_und_galax = [](const string& para_values) -> OrderedSet<int32_t> {
        OrderedSet<int32_t> result;
        auto parts = split(para_values, ',');
        
        for (const auto& part : parts) {
            string trimmed = trim(part);
            if (trimmed.empty()) continue;
            
            try {
                int32_t num = stoi(trimmed);
                int32_t abs_num = abs(num);
                if (abs_num != 0 && abs_num != 1) {
                    result.insert(abs_num);
                }
            } catch (...) {
                // Ignore non-numeric values
            }
        }
        
        return result;
    };
    
    auto lambda_prim_galax = [](const string& para_values) -> OrderedSet<int32_t> {
        OrderedSet<int32_t> result;
        auto parts = split(para_values, ',');
        
        for (const auto& part : parts) {
            string trimmed = trim(part);
            if (trimmed.empty()) continue;
            
            try {
                int32_t num = stoi(trimmed);
                int32_t abs_num = abs(num);
                
                // Check if prime
                bool is_prime = true;
                if (abs_num <= 1) is_prime = false;
                for (int32_t i = 2; i * i <= abs_num; ++i) {
                    if (abs_num % i == 0) {
                        is_prime = false;
                        break;
                    }
                }
                
                if (is_prime && abs_num != 0 && abs_num != 1) {
                    result.insert(abs_num);
                }
            } catch (...) {
                // Ignore non-numeric values
            }
        }
        
        return result;
    };
    
    // Main parameter commands
    map<string, int32_t> main_para_cmds = {
        {"zeilen", 0},
        {"spalten", 1},
        {"kombination", 2},
        {"ausgabe", 3}
    };
    
    int32_t last_main_cmd = -1;
    
    for (const auto& cmd : argv_) {
        if (cmd.size() > 1 && cmd[0] == '-' && cmd[1] != '-') {
            string cmd_body = cmd.substr(1);
            auto it = main_para_cmds.find(cmd_body);
            if (it != main_para_cmds.end()) {
                last_main_cmd = it->second;
            }
        } else if (starts_with(cmd, "--")) {
            string cmd_body = cmd.substr(2);
            
            switch (last_main_cmd) {
                case 1: { // spalten
                    if (breite_breiten_sys_argv_para(cmd_body, neg)) {
                        break;
                    }
                    
                    if (cmd_body == "keinenummerierung" && neg.empty()) {
                        tables_.set_nummeriere(false);
                        break;
                    }
                    
                    size_t eq_pos = cmd_body.find('=');
                    if (eq_pos != string::npos) {
                        string cmd_name = cmd_body.substr(0, eq_pos);
                        string cmd_values = cmd_body.substr(eq_pos + 1);
                        
                        auto values = split(cmd_values, ',');
                        for (const auto& value : values) {
                            string actual_value = value;
                            bool yes1 = false;
                            
                            if (!value.empty() && value[0] == '-') {
                                actual_value = value.substr(1);
                                yes1 = (neg == "-");
                            } else {
                                yes1 = neg.empty();
                            }
                            
                            if (yes1) {
                                // Look up parameter
                                auto key = make_pair(cmd_name, actual_value);
                                auto it = para_dict_.find(key);
                                if (it != para_dict_.end()) {
                                    // Process parameter data
                                    // Simplified implementation
                                }
                            }
                        }
                    } else {
                        // Command without value
                        auto key = make_pair(cmd_body, "");
                        auto it = para_dict_.find(key);
                        if (it != para_dict_.end()) {
                            // Process parameter data
                        }
                    }
                    break;
                }
                    
                case 2: { // kombination
                    string gal_wort = i18n::get_kombi_main_paras().galaxie + "=";
                    string uni_wort = i18n::get_kombi_main_paras().universum + "=";
                    
                    if (starts_with(cmd_body, gal_wort) || 
                        starts_with(cmd_body, uni_wort)) {
                        
                        size_t eq_pos = cmd_body.find('=');
                        string cmd_type = cmd_body.substr(0, eq_pos);
                        string values = cmd_body.substr(eq_pos + 1);
                        
                        auto value_list = split(values, ',');
                        for (const auto& value : value_list) {
                            string actual_value = value;
                            bool yes1 = false;
                            
                            if (!value.empty() && value[0] == '-') {
                                actual_value = value.substr(1);
                                yes1 = (neg == "-");
                            } else {
                                yes1 = neg.empty();
                            }
                            
                            if (yes1) {
                                OrderedSet<int32_t> spalten_set;
                                if (cmd_type == i18n::get_kombi_main_paras().galaxie) {
                                    auto it = kombi_reverse_dict_.find(actual_value);
                                    if (it != kombi_reverse_dict_.end()) {
                                        spalten_set.insert(it->second);
                                    }
                                } else {
                                    auto it = kombi_reverse_dict2_.find(actual_value);
                                    if (it != kombi_reverse_dict2_.end()) {
                                        spalten_set.insert(it->second);
                                    }
                                }
                                
                                // Create parameter data tuple
                                vector<OrderedSet<int32_t>> tupl(12);
                                if (cmd_type == i18n::get_kombi_main_paras().galaxie) {
                                    tupl[3] = spalten_set; // kombi1
                                } else {
                                    tupl[8] = spalten_set; // kombi2
                                }
                                
                                // Process the tuple
                                // Simplified implementation
                            }
                        }
                    }
                    break;
                }
                    
                default:
                    break;
            }
        }
    }
    
    if (neg.empty()) {
        produce_all_spalten_numbers("-");
        spalten_remove_doubles_n_then_remove_one_from_another();
    }
}

void Program::spalten_remove_doubles_n_then_remove_one_from_another() {
    for (int el2_type = 0; el2_type < 12; ++el2_type) {
        auto pos_key = make_pair(0, el2_type);
        auto neg_key = make_pair(1, el2_type);
        
        auto pos_it = spalten_arten_key_spaltennummern_value_.find(pos_key);
        auto neg_it = spalten_arten_key_spaltennummern_value_.find(neg_key);
        
        if (pos_it != spalten_arten_key_spaltennummern_value_.end() &&
            neg_it != spalten_arten_key_spaltennummern_value_.end()) {
            
            OrderedSet<int32_t> diff;
            set_difference(
                pos_it->second.begin(), pos_it->second.end(),
                neg_it->second.begin(), neg_it->second.end(),
                inserter(diff, diff.begin())
            );
            
            spalten_arten_key_spaltennummern_value_[pos_key] = diff;
        }
    }
    
    // Remove negative columns
    for (int el2_type = 0; el2_type < 12; ++el2_type) {
        spalten_arten_key_spaltennummern_value_.erase({1, el2_type});
    }
}

bool Program::breite_breiten_sys_argv_para(const string& cmd, const string& neg) {
    string para_breite = i18n::get_ausgabe_paras().breite + "=";
    string para_breite_n = i18n::get_ausgabe_paras().breiten + "=";
    
    if (starts_with(cmd, para_breite) && neg.empty()) {
        string value_str = cmd.substr(para_breite.size());
        
        if (breite_has_been_once_zero_) {
            tables_.set_text_width(0);
            breite_or_breiten_ = true;
            return true;
        }
        
        try {
            int32_t breite = abs(stoi(value_str));
            if (breite == 0) {
                breite_has_been_once_zero_ = true;
                tables_.set_text_width(0);
            } else {
                int32_t current_width = tables_.text_width();
                tables_.set_text_width(max(breite, current_width));
            }
            breite_or_breiten_ = true;
        } catch (...) {
            // Invalid number
        }
        return true;
    } else if (starts_with(cmd, para_breite_n) && neg.empty()) {
        string values = cmd.substr(para_breite_n.size());
        auto breiten_str = split(values, ',');
        vector<int32_t> breiten;
        
        for (const auto& s : breiten_str) {
            try {
                breiten.push_back(stoi(s));
            } catch (...) {
                // Invalid number
            }
        }
        
        tables_.set_breitenn(breiten);
        breite_or_breiten_ = true;
        return true;
    }
    
    return false;
}

void Program::store_parameters_for_columns() {
    // Simplified implementation
    // In a full implementation, this would populate para_dict_ and related structures
}

tuple<OrderedSet<string>, OrderedSet<int32_t>, OrderedSet<int32_t>,
      vector<int32_t>, OrderedSet<int32_t>, OrderedSet<int32_t>>
Program::parameters_to_commands_and_numbers(const vector<string>& argv, 
                                          const string& neg) {
    OrderedSet<string> param_lines;
    OrderedSet<int32_t> rows_as_numbers;
    OrderedSet<int32_t> rows_of_combi;
    vector<int32_t> spaltenreihenfolge_und_nur_diese;
    OrderedSet<int32_t> puniverseprims_only;
    OrderedSet<int32_t> gener_rows;
    
    bool in_zeilen_section = false;
    bool in_ausgabe_section = false;
    
    for (const auto& arg : argv) {
        if (arg.size() > 1 && arg[0] == '-' && arg[1] != '-') {
            // Main parameters
            if (arg == "-zeilen") {
                in_zeilen_section = true;
                in_ausgabe_section = false;
                big_parameter_.push_back("zeilen");
            } else if (arg == "-spalten") {
                in_zeilen_section = false;
                in_ausgabe_section = false;
                big_parameter_.push_back("spalten");
            } else if (arg == "-ausgabe") {
                in_zeilen_section = false;
                in_ausgabe_section = true;
                big_parameter_.push_back("ausgabe");
            } else if (arg == "-kombination") {
                in_zeilen_section = false;
                in_ausgabe_section = false;
                big_parameter_.push_back("kombination");
            } else if ((arg == "-h" || arg == "-help") && neg.empty()) {
                help_page();
            }
        } else if (starts_with(arg, "--")) {
            string cmd = arg.substr(2);
            
            if (in_zeilen_section) {
                auto& zeilen_paras = i18n::get_zeilen_paras();
                
                if (cmd == zeilen_paras.alles && neg.empty()) {
                    param_lines.insert("all");
                    ob_zeilen_bereiche_angegeben_ = true;
                } else {
                    size_t eq_pos = cmd.find('=');
                    if (eq_pos != string::npos) {
                        string cmd_name = cmd.substr(0, eq_pos);
                        string cmd_value = cmd.substr(eq_pos + 1);
                        
                        if (cmd_name == zeilen_paras.zeit) {
                            ob_zeilen_bereiche_angegeben_ = true;
                            auto subparas = split(cmd_value, ',');
                            for (const auto& subpara : subparas) {
                                if (subpara == neg + zeilen_paras.heute) {
                                    param_lines.insert("=");
                                } else if (subpara == neg + zeilen_paras.gestern) {
                                    param_lines.insert("<");
                                } else if (subpara == neg + zeilen_paras.morgen) {
                                    param_lines.insert(">");
                                }
                            }
                        } else if (cmd_name == zeilen_paras.zaehlung && neg.empty()) {
                            ob_zeilen_bereiche_angegeben_ = true;
                            auto lines = tables_.prepare().parameters_cmd_with_some_bereich(
                                cmd_value, "n", "", true);
                            param_lines.insert(lines.begin(), lines.end());
                        }
                        // Handle other zeilen parameters...
                    } else {
                        // Boolean parameters without value
                        if (cmd == zeilen_paras.invertieren && neg.empty()) {
                            ob_zeilen_bereiche_angegeben_ = true;
                            auto lines = tables_.prepare().parameters_cmd_with_some_bereich(
                                "1", "i", neg, true);
                            param_lines.insert(lines.begin(), lines.end());
                        }
                    }
                }
            } else if (in_ausgabe_section) {
                if (breite_breiten_sys_argv_para(cmd, neg)) {
                    continue;
                }
                
                auto& ausgabe_paras = i18n::get_ausgabe_paras();
                
                if (cmd == ausgabe_paras.keineueberschriften) {
                    tables_.set_keine_ueberschriften(true);
                } else if (cmd == ausgabe_paras.keinenummerierung) {
                    tables_.set_nummeriere(false);
                } else if (cmd == ausgabe_paras.keineleereninhalte) {
                    keine_leeren_inhalte_ = true;
                    tables_.set_keine_leeren_inhalte(true);
                } else {
                    size_t eq_pos = cmd.find('=');
                    if (eq_pos != string::npos) {
                        string cmd_name = cmd.substr(0, eq_pos);
                        string cmd_value = cmd.substr(eq_pos + 1);
                        
                        if (cmd_name == ausgabe_paras.spaltenreihenfolgeundnurdiese) {
                            auto numbers = utils::parse_range(cmd_value);
                            spaltenreihenfolge_und_nur_diese.assign(numbers.begin(), 
                                                                   numbers.end());
                        } else if (cmd_name == ausgabe_paras.art) {
                            string breite_ist_null = "--" + ausgabe_paras.breite + "=0";
                            
                            if (cmd_value == "shell") {
                                tables_.set_syntax_type(SyntaxType::Default);
                            } else if (cmd_value == "nichts") {
                                tables_.set_syntax_type(SyntaxType::Nichts);
                            } else if (cmd_value == "csv") {
                                tables_.set_syntax_type(SyntaxType::Csv);
                                tables_.output().set_one_table(true);
                                breite_breiten_sys_argv_para(breite_ist_null.substr(2), "");
                            } else if (cmd_value == "bbcode") {
                                html_or_bbcode_ = true;
                                tables_.set_syntax_type(SyntaxType::BBCode);
                            } else if (cmd_value == "html") {
                                html_or_bbcode_ = true;
                                tables_.set_syntax_type(SyntaxType::Html);
                            } else if (cmd_value == "emacs") {
                                tables_.output().set_one_table(true);
                                tables_.set_syntax_type(SyntaxType::Emacs);
                                breite_breiten_sys_argv_para(breite_ist_null.substr(2), "");
                            } else if (cmd_value == "markdown") {
                                tables_.output().set_one_table(true);
                                tables_.set_syntax_type(SyntaxType::Markdown);
                                breite_breiten_sys_argv_para(breite_ist_null.substr(2), "");
                            }
                        }
                    } else {
                        // Boolean ausgabe parameters
                        if ((cmd == ausgabe_paras.nocolor || cmd == ausgabe_paras.justtext) 
                            && neg.empty()) {
                            tables_.output().set_color(false);
                        } else if ((cmd == ausgabe_paras.endlessscreen || 
                                   cmd == ausgabe_paras.endless ||
                                   cmd == ausgabe_paras.dontwrap ||
                                   cmd == ausgabe_paras.onetable) && neg.empty()) {
                            tables_.output().set_one_table(true);
                        }
                    }
                }
            }
        }
    }
    
    if (!tables_.output().one_table()) {
        int32_t shell_rows_amount = utils::shell_rows_amount();
        int32_t text_width = tables_.text_width();
        
        if (shell_rows_amount > text_width + 7 || shell_rows_amount <= 0) {
            tables_.set_text_width(text_width);
        } else {
            tables_.set_text_width(shell_rows_amount - 7);
        }
    }
    
    tables_.set_if_zeilen_setted(ob_zeilen_bereiche_angegeben_);
    
    return {param_lines, rows_as_numbers, rows_of_combi, 
            spaltenreihenfolge_und_nur_diese, puniverseprims_only, gener_rows};
}

void Program::help_page() const {
    cout << i18n::RETA_HILFE << endl;
}

pair<vector<int32_t>, bool> Program::oberes_maximum_arg(const string& arg) const {
    vector<int32_t> werte;
    
    string oberesmaximum_str = i18n::get_zeilen_paras().oberesmaximum + "=";
    string vorhervonausschnitt_str = i18n::get_zeilen_paras().vorhervonausschnitt + "=";
    
    if (starts_with(arg, oberesmaximum_str)) {
        string value_str = arg.substr(oberesmaximum_str.size());
        try {
            werte.push_back(stoi(value_str));
            return {werte, true};
        } catch (...) {
            // Invalid number
        }
    } else if (starts_with(arg, vorhervonausschnitt_str)) {
        string value_str = arg.substr(vorhervonausschnitt_str.size());
        // Simplified implementation
        try {
            werte.push_back(max(1024, stoi(value_str)));
            return {werte, false};
        } catch (...) {
            // Invalid number
        }
    }
    
    return {{}, false};
}

optional<int32_t> Program::oberes_maximum2(const vector<string>& argv2) const {
    vector<int32_t> werte = {tables_.hoechste_zeile().first};
    
    for (const auto& arg : argv2) {
        auto [new_werte, _] = oberes_maximum_arg(arg);
        werte.insert(werte.end(), new_werte.begin(), new_werte.end());
    }
    
    if (werte.empty()) {
        return nullopt;
    }
    
    return *max_element(werte.begin(), werte.end());
}

bool Program::oberes_maximum(const string& arg) {
    auto [liste, wahrheitswert] = oberes_maximum_arg(arg);
    if (liste.empty() || !wahrheitswert) {
        return false;
    }
    
    int32_t max_val = *max_element(liste.begin(), liste.end());
    max_val = max(max_val, tables_.hoechste_zeile().first);
    tables_.set_hoechste_zeile(max_val);
    return true;
}

Table Program::combi_table_workflow(
    const Table& animals_professions_table,
    const OrderedSet<int32_t>& finally_display_lines,
    const vector<vector<int32_t>>& kombi_table_kombis,
    const pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>& 
        maintable2subtable_relation,
    const Table& new_table,
    const pair<vector<int32_t>, vector<int32_t>>& old2new_table,
    const OrderedSet<string>& param_lines,
    const string& csv_file_name) {
    
    // Simplified implementation
    return new_table;
}
