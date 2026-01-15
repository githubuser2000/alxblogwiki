#ifndef RETA_TABLES_HPP
#define RETA_TABLES_HPP

#include "types.hpp"
#include "error.hpp"
#include "i18n.hpp"
#include <memory>
#include <functional>

// Forward declarations
class Tables;
class Output;
class Prepare;
class Combi;
class MainTable;
class Concat;

// Output class
class Output {
private:
    std::string txt_;
    Vector<std::string> resulting_table_;
    bool color_ = true;
    bool one_table_ = false;
    Vector<int32_t> breiten_;
    bool nummerierung_ = true;
    int32_t textheight_ = 0;
    int32_t textwidth_ = 21;
    Vector<int32_t> religion_numbers_;
    OrderedSet<int32_t> rows_as_numbers_;
    Tables* tables_ = nullptr;
    
public:
    explicit Output(const std::string& txt);
    
    void set_tables(Tables* tables);
    Tables* tables() const { return tables_; }
    
    bool color() const { return color_; }
    void set_color(bool value) { color_ = value; }
    
    bool one_table() const { return one_table_; }
    void set_one_table(bool value) { one_table_ = value; }
    
    const OrderedSet<int32_t>& rows_as_numbers() const { return rows_as_numbers_; }
    void set_rows_as_numbers(const OrderedSet<int32_t>& value) { rows_as_numbers_ = value; }
    
    Table only_that_columns(const Table& table, const Vector<size_t>& only_that_columns) const;
    
    Vector<std::string> cli_out(const OrderedSet<int32_t>& finally_display_lines,
                               const Table& new_table,
                               size_t numlen,
                               const std::pair<size_t, size_t>& rows_range);
    
    const Vector<std::string>& resulting_table() const { return resulting_table_; }
};

// Prepare class
class Prepare {
private:
    bool ifprimmultis_ = false;
    bool if_zeilen_setted_ = false;
    Vector<int32_t> breiten_;
    bool nummerierung_ = true;
    int32_t text_width_ = 0;
    Vector<int32_t> religion_numbers_;
    OrderedSet<int32_t> rows_as_numbers_;
    Tables* tables_ = nullptr;
    
public:
    Prepare();
    
    void set_tables(Tables* tables);
    Tables* tables() const { return tables_; }
    
    bool ifprimmultis() const { return ifprimmultis_; }
    void set_ifprimmultis(bool value) { ifprimmultis_ = value; }
    
    bool if_zeilen_setted() const { return if_zeilen_setted_; }
    void set_if_zeilen_setted(bool value) { if_zeilen_setted_ = value; }
    
    const OrderedSet<int32_t>& rows_as_numbers() const { return rows_as_numbers_; }
    void set_rows_as_numbers(const OrderedSet<int32_t>& value) { rows_as_numbers_ = value; }
    
    OrderedSet<std::string> parameters_cmd_with_some_bereich(
        const std::string& s,
        const std::string& cmd_type,
        const std::string& neg,
        bool keine_neg_beruecksichtigung) const;
    
    std::pair<OrderedSet<std::string>, OrderedSet<std::string>> 
    delete_doubles_in_sets(const OrderedSet<std::string>& set1,
                          const OrderedSet<std::string>& set2) const;
    
    std::tuple<OrderedSet<int32_t>, size_t, Table, size_t, std::pair<size_t, size_t>>
    prepare4out_before_for_loop_spalten_zeilen_bestimmen(
        const Table& relitable,
        const OrderedSet<std::string>& param_lines,
        const OrderedSet<std::string>& param_lines_not) const;
    
    std::tuple<OrderedSet<int32_t>, Table, size_t, 
               std::pair<size_t, size_t>, std::pair<Vector<int32_t>, Vector<int32_t>>>
    prepare4out(const OrderedSet<std::string>& param_lines,
               const OrderedSet<std::string>& param_lines_not,
               const Table& relitable,
               const OrderedSet<int32_t>& rows_as_numbers,
               const Map<std::string, OrderedMap<int32_t, Vector<std::string>>>& gebr_spalten,
               const OrderedMap<int32_t, Vector<std::string>>& prim_spalten) const;
    
    int32_t zeile_which_zaehlung(int32_t zeile) const;
    Vector<std::string> cell_work(const std::string& content, int32_t certain_text_width) const;
};

// Combi class
class Combi {
private:
    int32_t sum_of_all_combi_rows_amount_ = 0;
    Vector<int32_t> religion_numbers_;
    OrderedSet<int32_t> rows_of_combi_;
    Tables* tables_ = nullptr;
    
public:
    Combi();
    
    void set_tables(Tables* tables);
    Tables* tables() const { return tables_; }
    
    int32_t sum_of_all_combi_rows_amount() const { return sum_of_all_combi_rows_amount_; }
    
    OrderedMap<int32_t, OrderedSet<int32_t>> prepare_kombi(
        const OrderedSet<int32_t>& finally_display_lines,
        const Table& kombi_table,
        const OrderedSet<std::string>& param_lines,
        const OrderedSet<int32_t>& displaying_zeilen,
        const Vector<Vector<int32_t>>& kombi_table_kombis) const;
    
    Vector<OrderedMap<int32_t, Table>> prepare_table_join(
        const OrderedMap<int32_t, OrderedSet<int32_t>>& chosen_kombi_lines,
        const Table& new_table_kombi) const;
    
    Table table_join(Table main_table,
                    const Vector<OrderedMap<int32_t, Table>>& many_sub_tables,
                    const std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t
