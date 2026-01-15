// ===============================
// ansi_i18n.cpp
// ===============================
#include "py_runtime.hpp"
#include <map>
#include <cstdlib>

// ---------- ANSI ----------
PyValue py_ansi(std::vector<PyValue> args, PyFrame&){
    std::string text = (args.size()>0 && args[0].type==PyValue::STR)?*args[0].s:"";
    std::string color = (args.size()>1 && args[1].type==PyValue::STR)?*args[1].s:"";
    bool bold = (args.size()>2 && args[2].type==PyValue::BOOL && args[2].b);

    std::map<std::string,std::string> col{
        {"black","30"},{"red","31"},{"green","32"},{"yellow","33"},
        {"blue","34"},{"magenta","35"},{"cyan","36"},{"white","37"}
    };

    std::string code;
    if(bold) code+="1;";
    if(col.count(color)) code+=col[color];

    if(code.empty()) return PyValue::Str(text);
    return PyValue::Str("\x1b["+code+"m"+text+"\x1b[0m");
}

// ---------- i18n ----------
static std::string CURRENT_LANG="en";
static std::unordered_map<std::string,
        std::unordered_map<std::string,std::string>> WORDS;

PyValue py_set_lang(std::vector<PyValue> args, PyFrame&){
    if(!args.empty() && args[0].type==PyValue::STR)
        CURRENT_LANG=*args[0].s;
    return PyValue::None();
}

PyValue py_add_words(std::vector<PyValue> args, PyFrame&){
    if(args.size()<2) return PyValue::None();
    if(args[0].type!=PyValue::STR || args[1].type!=PyValue::DICT)
        return PyValue::None();

    auto& d = WORDS[*args[0].s];
    for(auto& kv : *args[1].dict){
        if(kv.second.type==PyValue::STR)
            d[kv.first]=*kv.second.s;
    }
    return PyValue::None();
}

PyValue py_get_word(std::vector<PyValue> args, PyFrame&){
    if(args.empty()||args[0].type!=PyValue::STR)
        return PyValue::None();
    auto itL = WORDS.find(CURRENT_LANG);
    if(itL!=WORDS.end()){
        auto it = itL->second.find(*args[0].s);
        if(it!=itL->second.end())
            return PyValue::Str(it->second);
    }
    return PyValue::Str(*args[0].s);
}
