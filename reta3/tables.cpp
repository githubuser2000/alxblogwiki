#include "tables.hpp"
#include <utility>

// Output implementation
Output::Output(const std::string& txt) : txt_(txt) {}

void Output::set_tables(Tables* tables) {
    tables_ = tables;
}

Table Output::only_that_columns(const Table& table, 
                               const Vector<size_t>& only_that_columns) const {
    if (only_that_columns.empty()) {
        return table;
    }
    
    Table new_table;
    for (const auto& row : table) {
        Row new_row;
        for (size_t col_idx : only_that_columns) {
            if (col_idx > 0 && col_idx <= row.size()) {
                new_row.push_back(row[col_idx - 1]);
            }
        }
        if (!new_row.empty()) {
            new_table.push_back(new_row);
        }
    }
    
    if (new_table.empty()) {
        return table;
    }
    return new_table;
}

Vector<std::string> Output::cli_out(const OrderedSet<int32_t>& finally_display_lines,
                                   const Table& new_table,
                                   size_t numlen,
                                   const std::pair<size_t, size_t>& rows_range) {
    Vector<std::string> result;
    
    // Simple implementation for now
    for (size_t row_idx = 0; row_idx < new_table.size(); ++row_idx) {
        std::string line;
        
        if (nummerierung_ && row_idx > 0) {
            char buffer[32];
            snprintf(buffer, sizeof(buffer), "%*zu", (int)numlen, row_idx);
            line += buffer;
            line += " ";
        }
        
        const auto& row = new_table[row_idx];
        for (size_t col_idx = 0; col_idx < row.size(); ++col_idx) {
            if (col_idx > 0) line += " | ";
            
            const auto& cell = row[col_idx];
            if (!cell.lines.empty()) {
                line += cell.lines[0];
            }
        }
        
        result.push_back(line);
    }
    
    resulting_table_ = result;
    return result;
}

// Prepare implementation
Prepare::Prepare() = default;

void Prepare::set_tables(Tables* tables) {
    tables_ = tables;
}

OrderedSet<std::string> Prepare::parameters_cmd_with_some_bereich(
    const std::string& s,
    const std::string& cmd_type,
    const std::string& neg,
    bool keine_neg_beruecksichtigung) const {
    OrderedSet<std::string> result;
    
    auto parts = utils::split(s, ',');
    for (const auto& part : parts) {
        if (part.empty()) continue;
        
        if (cmd_type == "n") {
            auto range = RangeSpec::parse(part);
            auto numbers = range.to_numbers(1000);
            for (auto num : numbers) {
                result.insert(std::to_string(num));
            }
        } else if (cmd_type == "^") {
            try {
                int32_t num = std::stoi(part);
                result.insert(std::to_string(num) + "^2");
            } catch (...) {
                // Ignore invalid numbers
            }
        } else if (cmd_type == "b") {
            try {
                int32_t num = std::stoi(part);
                result.insert(std::to_string(num) + "b");
            } catch (...) {
                // Ignore invalid numbers
            }
        } else {
            result.insert(part);
        }
    }
    
    return result;
}

std::pair<OrderedSet<std::string>, OrderedSet<std::string>> 
Prepare::delete_doubles_in_sets(const OrderedSet<std::string>& set1,
                               const OrderedSet<std::string>& set2) const {
    OrderedSet<std::string> diff1, diff2;
    
    std::set_difference(set1.begin(), set1.end(),
                       set2.begin(), set2.end(),
                       std::inserter(diff1, diff1.begin()));
    
    std::set_difference(set2.begin(), set2.end(),
                       set1.begin(), set1.end(),
                       std::inserter(diff2, diff2.begin()));
    
    return {diff1, diff2};
}

std::tuple<OrderedSet<int32_t>, size_t, Table, size_t, std::pair<size_t, size_t>>
Prepare::prepare4out_before_for_loop_spalten_zeilen_bestimmen(
    const Table& relitable,
    const OrderedSet<std::string>& param_lines,
    const OrderedSet<std::string>& param_lines_not) const {
    OrderedSet<int32_t> finally_display_lines;
    for (int32_t i = 0; i < static_cast<int32_t>(relitable.size()); ++i) {
        finally_display_lines.insert(i);
    }
    
    size_t headings_amount = relitable.empty() ? 0 : relitable[0].size();
    size_t numlen = 3;
    std::pair<size_t, size_t> rows_range = {0, 10};
    
    return {finally_display_lines, headings_amount, relitable, numlen, rows_range};
}

std::tuple<OrderedSet<int32_t>, Table, size_t, 
           std::pair<size_t, size_t>, std::pair<Vector<int32_t>, Vector<int32_t>>>
Prepare::prepare4out(const OrderedSet<std::string>& param_lines,
                   const OrderedSet<std::string>& param_lines_not,
                   const Table& relitable,
                   const OrderedSet<int32_t>& rows_as_numbers,
                   const Map<std::string, OrderedMap<int32_t, Vector<std::string>>>& gebr_spalten,
                   const OrderedMap<int32_t, Vector<std::string>>& prim_spalten) const {
    OrderedSet<int32_t> finally_display_lines;
    for (int32_t i = 0; i < static_cast<int32_t>(relitable.size()); ++i) {
        finally_display_lines.insert(i);
    }
    
    size_t numlen = 3;
    std::pair<size_t, size_t> rows_range = {0, 10};
    std::pair<Vector<int32_t>, Vector<int32_t>> old2new_table;
    
    return {finally_display_lines, relitable, numlen, rows_range, old2new_table};
}

int32_t Prepare::zeile_which_zaehlung(int32_t zeile) const {
    return zeile;
}

Vector<std::string> Prepare::cell_work(const std::string& content, 
                                      int32_t certain_text_width) const {
    if (certain_text_width <= 0) {
        return {content};
    }
    
    Vector<std::string> result;
    std::string current_line;
    size_t width = static_cast<size_t>(certain_text_width);
    
    std::stringstream ss(content);
    std::string word;
    
    while (ss >> word) {
        if (current_line.length() + word.length() + 1 > width && !current_line.empty()) {
            result.push_back(current_line);
            current_line.clear();
        }
        
        if (!current_line.empty()) {
            current_line += " ";
        }
        current_line += word;
    }
    
    if (!current_line.empty()) {
        result.push_back(current_line);
    }
    
    return result;
}

// Combi implementation
Combi::Combi() = default;

void Combi::set_tables(Tables* tables) {
    tables_ = tables;
}

OrderedMap<int32_t, OrderedSet<int32_t>> Combi::prepare_kombi(
    const OrderedSet<int32_t>& finally_display_lines,
    const Table& kombi_table,
    const OrderedSet<std::string>& param_lines,
    const OrderedSet<int32_t>& displaying_zeilen,
    const Vector<Vector<int32_t>>& kombi_table_kombis) const {
    OrderedMap<int32_t, OrderedSet<int32_t>> chosen_kombi_lines;
    
    for (const auto& condition : param_lines) {
        if (condition == "ka" || condition == "ka2") {
            for (size_t kombi_line_number = 0; 
                 kombi_line_number < kombi_table_kombis.size(); 
                 ++kombi_line_number) {
                for (int32_t kombi_number : kombi_table_kombis[kombi_line_number]) {
                    if (displaying_zeilen.find(kombi_number) != displaying_zeilen.end()) {
                        chosen_kombi_lines[kombi_number].insert(
                            static_cast<int32_t>(kombi_line_number) + 1);
                    }
                }
            }
        }
    }
    
    return chosen_kombi_lines;
}

Vector<OrderedMap<int32_t, Table>> Combi::prepare_table_join(
    const OrderedMap<int32_t, OrderedSet<int32_t>>& chosen_kombi_lines,
    const Table& new_table_kombi) const {
    Vector<OrderedMap<int32_t, Table>> kombi_tables;
    
    for (const auto& [key, value] : chosen_kombi_lines) {
        OrderedMap<int32_t, Table> tables_map;
        
        for (int32_t kombi_line_number : value) {
            Set<size_t> lines_allowed = {static_cast<size_t>(kombi_line_number)};
            auto into = tables_->table_reduced_in_lines_by_type_set(new_table_kombi, lines_allowed);
            
            if (!into.empty()) {
                tables_map[key].push_back(into[0]);
            }
        }
        
        if (!tables_map.empty()) {
            kombi_tables.push_back(tables_map);
        }
    }
    
    return kombi_tables;
}

Table Combi::table_join(Table main_table,
                       const Vector<OrderedMap<int32_t, Table>>& many_sub_tables,
                       const std::pair<OrderedMap<int32_t, int32_t>, 
                                       OrderedMap<int32_t, int32_t>>& 
                           maintable2subtable_relation,
                       const std::pair<Vector<int32_t>, Vector<int32_t>>& old2new_rows,
                       const OrderedSet<int32_t>& rows_of_combi) const {
    // Simplified implementation - just return the main table
    return main_table;
}

std::tuple<Table, Table, Vector<Vector<int32_t>>, 
           std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>>
Combi::read_kombi_csv(const Table& relitable,
                     OrderedSet<int32_t>& rows_as_numbers,
                     const OrderedSet<int32_t>& rows_of_combi,
                     const std::string& csv_file_name) const {
    // Simplified implementation
    return {Table(), relitable, Vector<Vector<int32_t>>(), 
            {OrderedMap<int32_t, int32_t>(), OrderedMap<int32_t, int32_t>()}};
}

// MainTable implementation
MainTable::MainTable() = default;

void MainTable::set_tables(Tables* tables) {
    tables_ = tables;
}

void MainTable::create_spalte_gestirn(Table& relitable, 
                                     OrderedSet<int32_t>& rows_as_numbers) const {
    if (rows_as_numbers.find(64) != rows_as_numbers.end() && !relitable.empty()) {
        // Add header
        relitable[0].push_back(Cell("Gestirn"));
        
        // Add content for each row
        for (size_t i = 0; i < relitable.size(); ++i) {
            if (i == 0) continue; // Skip header row
            
            std::string content = (i % 2 == 0) ? "Sonne" : "Mond";
            relitable[i].push_back(Cell(content));
        }
        
        // Add to rows_as_numbers
        rows_as_numbers.insert(static_cast<int32_t>(relitable[0].size()) - 1);
    }
}

// Concat implementation
Concat::Concat() = default;

void Concat::set_tables(Tables* tables) {
    tables_ = tables;
}

std::tuple<Table, OrderedSet<int32_t>, OrderedMap<int32_t, Vector<std::string>>>
Concat::read_concat_csv(const Table& relitable,
                       const OrderedSet<int32_t>& rows_as_numbers,
                       const OrderedSet<int32_t>& input,
                       int32_t i) const {
    // Simplified implementation
    return {relitable, rows_as_numbers, OrderedMap<int32_t, Vector<std::string>>()};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_vervielfache_zeile(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_modallogik(
    const Table& relitable,
    const OrderedSet<int32_t>& gener_rows,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_prim_creativity_type(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_gleichheit_freiheit_dominieren(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_geist_emotion_energie_materie_topologie(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_mond_exponzieren_logarithmus_typ(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat1_row_prim_universe2(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers,
    const OrderedSet<int32_t>& spalten,
    const OrderedMap<std::string, Vector<Vector<std::pair<std::string, std::string>>>>& 
        para_text_namen) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat1_primzahlkreuz_pro_contra(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers,
    const OrderedSet<int32_t>& spalten,
    const i18n::ParametersMain& parameters_main) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::concat_love_polygon(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::spalte_fuer_gegen_innen_aussen_seitlich_prim(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers) const {
    return {relitable, rows_as_numbers};
}

std::pair<Table, OrderedSet<int32_t>> Concat::spalte_meta_kontret_theorie_abstrakt_etc_1(
    const Table& relitable,
    const OrderedSet<int32_t>& rows_as_numbers,
    const Vector<int32_t>& couples_x) const {
    return {relitable, rows_as_numbers};
}

// Tables main implementation
Tables::Tables(std::optional<int32_t> hoechst_zeil, const std::string& txt) 
    : output_(txt) {
    if (hoechst_zeil) {
        config_.hoechste_zeile = {*hoechst_zeil, *hoechst_zeil};
    }
    
    // Set references
    prepare_.set_tables(this);
    output_.set_tables(this);
    combi_.set_tables(this);
    main_table_.set_tables(this);
    concat_.set_tables(this);
}

void Tables::set_hoechste_zeile(int32_t value) {
    config_.hoechste_zeile = {value, value};
}

void Tables::set_text_width(int32_t value) {
    config_.text_width = value;
}

void Tables::set_nummeriere(bool value) {
    config_.nummeriere = value;
}

void Tables::set_breitenn(const Vector<int32_t>& value) {
    config_.breiten = value;
}

Table Tables::table_reduced_in_lines_by_type_set(
    const Table& table,
    const Set<size_t>& lines_allowed) const {
    Table new_table;
    for (size_t i = 0; i < table.size(); ++i) {
        if (lines_allowed.find(i) != lines_allowed.end()) {
            new_table.push_back(table[i]);
        }
    }
    return new_table;
}

std::pair<Vector<std::string>, Vector<std::string>> 
Tables::fill_both(Vector<std::string> liste1, Vector<std::string> liste2) {
    while (liste1.size() < liste2.size()) {
        liste1.push_back("");
    }
    while (liste2.size() < liste1.size()) {
        liste2.push_back("");
    }
    return {liste1, liste2};
}
