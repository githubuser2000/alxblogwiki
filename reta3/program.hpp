#ifndef RETA_PROGRAM_HPP
#define RETA_PROGRAM_HPP

#include "tables.hpp"
#include "types.hpp"
#include "i18n.hpp"
#include <vector>
#include <string>
#include <memory>
#include <functional>

class Program {
private:
    std::vector<std::string> argv_;
    Tables tables_;
    bool breite_has_been_once_zero_ = false;
    bool ob_zeilen_bereiche_angegeben_ = false;
    bool html_or_bbcode_ = false;
    bool breite_or_breiten_ = false;
    bool keine_leeren_inhalte_ = false;
    bool invert_alles_ = false;
    bool run_alles_ = false;
    std::vector<std::string> resulting_table_;
    
    // Parameter structures
    struct ParameterData {
        std::vector<OrderedSet<int32_t>> spalten;
        std::function<OrderedSet<int32_t>(const std::string&)> lambda_func;
    };
    
    OrderedMap<std::pair<std::string, std::string>, ParameterData> para_dict_;
    OrderedMap<std::string, std::vector<std::string>> para_main_dict_;
    std::vector<OrderedMap<int32_t, std::string>> data_dict_;
    OrderedMap<std::string, int32_t> kombi_reverse_dict_;
    OrderedMap<std::string, int32_t> kombi_reverse_dict2_;
    OrderedMap<std::pair<int32_t, int32_t>, OrderedSet<int32_t>> 
        spalten_arten_key_spaltennummern_value_;
    
    // Spalten type naming
    struct SpaltenTyp {
        std::pair<int32_t, int32_t> ordinary;
        std::pair<int32_t, int32_t> generated1;
        std::pair<int32_t, int32_t> concat1;
        std::pair<int32_t, int32_t> kombi1;
        std::pair<int32_t, int32_t> bool_and_tuple_set1;
        std::pair<int32_t, int32_t> gebro_uni1;
        std::pair<int32_t, int32_t> gebr_gal1;
        std::pair<int32_t, int32_t> generated2;
        std::pair<int32_t, int32_t> kombi2;
        std::pair<int32_t, int32_t> gebr_emo1;
        std::pair<int32_t, int32_t> gebr_groe1;
        std::pair<int32_t, int32_t> metakonkret;
        std::pair<int32_t, int32_t> ordinary_not;
        std::pair<int32_t, int32_t> generated1_not;
        std::pair<int32_t, int32_t> concat1_not;
        std::pair<int32_t, int32_t> kombi1_not;
        std::pair<int32_t, int32_t> bool_and_tuple_set1_not;
        std::pair<int32_t, int32_t> gebro_uni1_not;
        std::pair<int32_t, int32_t> gebr_gal1_not;
        std::pair<int32_t, int32_t> generated2_not;
        std::pair<int32_t, int32_t> kombi2_not;
        std::pair<int32_t, int32_t> gebr_emo1_not;
        std::pair<int32_t, int32_t> gebr_groe1_not;
        std::pair<int32_t, int32_t> metakonkret_not;
    } spalten_type_naming_;
    
    // Program state
    size_t rows_len_ = 0;
    Table relitable_;
    OrderedSet<int32_t> rows_as_numbers_;
    OrderedSet<int32_t> rows_of_combi_;
    OrderedSet<int32_t> rows_of_combi2_;
    OrderedSet<int32_t> gener_rows_;
    OrderedSet<int32_t> puniverseprims_;
    OrderedSet<int32_t> all_simple_command_spalten_;
    std::vector<std::string> big_parameter_;
    OrderedSet<int32_t> will_be_overwritten_rows_of_combi_;
    int32_t last_line_number_ = 0;
    int32_t spalten_vanilla_amount_ = 0;
    
public:
    Program(const std::vector<std::string>& argv, 
           const std::string& txt, 
           bool run_alles);
    
    void invert_alles();
    void run();
    
    const std::vector<std::string>& resulting_table() const { 
        return resulting_table_; 
    }
    
private:
    std::vector<std::string> workflow_everything(const std::vector<std::string>& argv);
    void produce_all_spalten_numbers(const std::string& neg = "");
    void spalten_remove_doubles_n_then_remove_one_from_another();
    bool breite_breiten_sys_argv_para(const std::string& cmd, const std::string& neg);
    void store_parameters_for_columns();
    
    std::tuple<OrderedSet<std::string>, OrderedSet<int32_t>, OrderedSet<int32_t>,
               std::vector<int32_t>, OrderedSet<int32_t>, OrderedSet<int32_t>>
    parameters_to_commands_and_numbers(const std::vector<std::string>& argv, 
                                      const std::string& neg = "");
    
    void help_page() const;
    
    std::tuple<size_t, OrderedSet<std::string>, OrderedSet<std::string>,
               Table, OrderedSet<int32_t>, Table, OrderedSet<int32_t>,
               std::vector<std::vector<int32_t>>,
               std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>,
               std::vector<int32_t>, OrderedMap<int32_t, std::vector<std::string>>,
               Map<std::string, OrderedMap<int32_t, std::vector<std::string>>>,
               Table, std::vector<std::vector<int32_t>>,
               std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>>
    bring_all_important_begin_things(const std::vector<std::string>& argv);
    
    std::pair<std::vector<int32_t>, bool> oberes_maximum_arg(const std::string& arg) const;
    std::optional<int32_t> oberes_maximum2(const std::vector<std::string>& argv2) const;
    bool oberes_maximum(const std::string& arg);
    
    Table combi_table_workflow(
        const Table& animals_professions_table,
        const OrderedSet<int32_t>& finally_display_lines,
        const std::vector<std::vector<int32_t>>& kombi_table_kombis,
        const std::pair<OrderedMap<int32_t, int32_t>, OrderedMap<int32_t, int32_t>>& 
            maintable2subtable_relation,
        const Table& new_table,
        const std::pair<std::vector<int32_t>, std::vector<int32_t>>& old2new_table,
        const OrderedSet<std::string>& param_lines,
        const std::string& csv_file_name);
};

#endif // RETA_PROGRAM_HPP
