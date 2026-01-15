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
                    const std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>& 
                        maintable2subtable_relation,
                    const std::pair<Vector<int32_t>, Vector<int32_t>>& old2new_rows,
                    const OrderedSet<int32_t>& rows_of_combi) const;
    
    std::tuple<Table, Table, Vector<Vector<int32_t>>, 
               std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>>
    read_kombi_csv(const Table& relitable,
                  OrderedSet<int32_t>& rows_as_numbers,
                  const OrderedSet<int32_t>& rows_of_combi,
                  const std::string& csv_file_name) const;
};

// MainTable class
class MainTable {
private:
    Tables* tables_ = nullptr;
    
public:
    MainTable();
    
    void set_tables(Tables* tables);
    
    void create_spalte_gestirn(Table& relitable, OrderedSet<int32_t>& rows_as_numbers) const;
};

// Concat class
class Concat {
private:
    Tables* tables_ = nullptr;
    Vector<int32_t> ones_;
    
public:
    Concat();
    
    void set_tables(Tables* tables);
    
    const Vector<int32_t>& ones() const { return ones_; }
    void set_ones(const Vector<int32_t>& ones) { ones_ = ones; }
    
    std::tuple<Table, OrderedSet<int32_t>, OrderedMap<int32_t, Vector<std::string>>>
    read_concat_csv(const Table& relitable,
                   const OrderedSet<int32_t>& rows_as_numbers,
                   const OrderedSet<int32_t>& input,
                   int32_t i) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_vervielfache_zeile(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_modallogik(
        const Table& relitable,
        const OrderedSet<int32_t>& gener_rows,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_prim_creativity_type(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_gleichheit_freiheit_dominieren(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_geist_emotion_energie_materie_topologie(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_mond_exponzieren_logarithmus_typ(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat1_row_prim_universe2(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers,
        const OrderedSet<int32_t>& spalten,
        const OrderedMap<std::string, Vector<Vector<std::pair<std::string, std::string>>>>& 
            para_text_namen) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat1_primzahlkreuz_pro_contra(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers,
        const OrderedSet<int32_t>& spalten,
        const i18n::ParametersMain& parameters_main) const;
    
    std::pair<Table, OrderedSet<int32_t>> concat_love_polygon(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> spalte_fuer_gegen_innen_aussen_seitlich_prim(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers) const;
    
    std::pair<Table, OrderedSet<int32_t>> spalte_meta_kontret_theorie_abstrakt_etc_1(
        const Table& relitable,
        const OrderedSet<int32_t>& rows_as_numbers,
        const Vector<int32_t>& couples_x) const;
};

// Main Tables class
class Tables {
private:
    TableConfig config_;
    ColorConfig color_config_;
    OrderedMap<int32_t, int32_t> row_num_display_to_orig_;
    OrderedMap<int32_t, std::unique_ptr<ParameterValue>> generated_spalten_parameter_;
    OrderedMap<int32_t, OrderedSet<SpaltenTag>> generated_spalten_parameter_tags_;
    Prepare prepare_;
    Output output_;
    Combi combi_;
    MainTable main_table_;
    Concat concat_;
    Vector<int32_t> religion_numbers_;
    OrderedSet<int32_t> gener_rows_;
    OrderedSet<int32_t> rows_of_combi_;
    int32_t spalten_vanilla_amount_ = 0;
    Vector<OrderedMap<int32_t, std::string>> data_dict_;
    SyntaxType syntax_type_ = SyntaxType::Default;
    bool keine_leeren_inhalte_ = false;
    bool keine_ueberschriften_ = false;
    bool spalte_gestirn_ = false;
    bool if_zeilen_setted_ = false;
    bool if_prim_multis_ = false;
    int32_t last_line_number_ = 0;
    
public:
    Tables(std::optional<int32_t> hoechst_zeil, const std::string& txt);
    
    // Getters
    const TableConfig& config() const { return config_; }
    TableConfig& config() { return config_; }
    
    const Prepare& prepare() const { return prepare_; }
    Prepare& prepare() { return prepare_; }
    
    const Output& output() const { return output_; }
    Output& output() { return output_; }
    
    const Combi& combi() const { return combi_; }
    Combi& combi() { return combi_; }
    
    const Concat& concat() const { return concat_; }
    Concat& concat() { return concat_; }
    
    std::pair<int32_t, int32_t> hoechste_zeile() const { return config_.hoechste_zeile; }
    void set_hoechste_zeile(int32_t value);
    
    int32_t text_width() const { return config_.text_width; }
    void set_text_width(int32_t value);
    
    int32_t text_height() const { return config_.text_height; }
    void set_text_height(int32_t value);
    
    bool nummeriere() const { return config_.nummeriere; }
    void set_nummeriere(bool value);
    
    bool keine_ueberschriften() const { return keine_ueberschriften_; }
    void set_keine_ueberschriften(bool value);
    
    bool keine_leeren_inhalte() const { return keine_leeren_inhalte_; }
    void set_keine_leeren_inhalte(bool value);
    
    bool spalte_gestirn() const { return spalte_gestirn_; }
    void set_spalte_gestirn(bool value);
    
    const Vector<int32_t>& breitenn() const { return config_.breiten; }
    void set_breitenn(const Vector<int32_t>& value);
    
    bool if_zeilen_setted() const { return if_zeilen_setted_; }
    void set_if_zeilen_setted(bool value);
    
    bool if_prim_multis() const { return if_prim_multis_; }
    void set_if_prim_multis(bool value);
    
    const OrderedSet<int32_t>& gener_rows() const { return gener_rows_; }
    void set_gener_rows(const OrderedSet<int32_t>& value);
    
    const Vector<int32_t>& religion_numbers() const { return religion_numbers_; }
    void set_religion_numbers(const Vector<int32_t>& value);
    
    const OrderedSet<int32_t>& rows_of_combi() const { return rows_of_combi_; }
    void set_rows_of_combi(const OrderedSet<int32_t>& value);
    
    int32_t spalten_vanilla_amount() const { return spalten_vanilla_amount_; }
    void set_spalten_vanilla_amount(int32_t value);
    
    const Vector<OrderedMap<int32_t, std::string>>& data_dict() const { return data_dict_; }
    void set_data_dict(const Vector<OrderedMap<int32_t, std::string>>& value);
    
    SyntaxType syntax_type() const { return syntax_type_; }
    void set_syntax_type(SyntaxType value);
    
    int32_t last_line_number() const { return last_line_number_; }
    void set_last_line_number(int32_t value);
    
    Table table_reduced_in_lines_by_type_set(
        const Table& table,
        const Set<size_t>& lines_allowed) const;
    
    static std::pair<Vector<std::string>, Vector<std::string>> 
    fill_both(Vector<std::string> liste1, Vector<std::string> liste2);
};

#endif // RETA_TABLES_HPP
