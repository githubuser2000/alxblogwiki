// ===============================
// eval_exec.cpp   (C++17)
// ===============================
#include "py_runtime.hpp"
#include <sstream>

/*
 * Ziel:
 *  - Python-ähnliches eval()/exec()
 *  - KEIN Parser, KEIN Optimierer
 *  - Nur das, was center.py faktisch benutzt:
 *      * Variablenzugriff
 *      * Funktionsaufruf
 *      * Literale (int, string)
 *  - Alles läuft über PyFrame.globals
 */

// -------------------------------------------------
// sehr primitiver Tokenizer
// -------------------------------------------------
static std::vector<std::string> tokenize(const std::string& src) {
    std::vector<std::string> out;
    std::string buf;
    bool in_str=false;

    for(char c: src){
        if(c=='"' && !in_str){
            in_str=true;
            buf.push_back(c);
        } else if(c=='"' && in_str){
            buf.push_back(c);
            out.push_back(buf);
            buf.clear();
            in_str=false;
        } else if(in_str){
            buf.push_back(c);
        } else if(isspace(c)){
            if(!buf.empty()){ out.push_back(buf); buf.clear(); }
        } else if(c=='('||c==')'||c==',' ){
            if(!buf.empty()){ out.push_back(buf); buf.clear(); }
            out.emplace_back(1,c);
        } else {
            buf.push_back(c);
        }
    }
    if(!buf.empty()) out.push_back(buf);
    return out;
}

// -------------------------------------------------
// eval(expr)
// -------------------------------------------------
PyValue py_eval(std::vector<PyValue> args, PyFrame& frame){
    if(args.empty() || args[0].type!=PyValue::STR)
        return PyValue::None();

    std::string expr = *args[0].s;
    auto toks = tokenize(expr);

    // 1) Literal int
    if(toks.size()==1){
        const std::string& t=toks[0];
        if(!t.empty() && std::isdigit(t[0]))
            return PyValue::Int(std::stoll(t));

        auto it=frame.globals.find(t);
        if(it!=frame.globals.end())
            return it->second;
    }

    // 2) func(arg)
    if(toks.size()>=4 && toks[1]=="("){
        std::string fname=toks[0];
        auto it=frame.globals.find(fname);
        if(it==frame.globals.end() || it->second.type!=PyValue::FUNC)
            return PyValue::Exc("NameError","function not found");

        std::vector<PyValue> call_args;
        for(size_t i=2;i<toks.size();++i){
            if(toks[i]==")") break;
            if(toks[i]==",") continue;

            if(std::isdigit(toks[i][0]))
                call_args.push_back(PyValue::Int(std::stoll(toks[i])));
            else if(toks[i].front()=='"' && toks[i].back()=='"')
                call_args.push_back(PyValue::Str(
                    toks[i].substr(1,toks[i].size()-2)
                ));
            else {
                auto vit=frame.globals.find(toks[i]);
                if(vit!=frame.globals.end())
                    call_args.push_back(vit->second);
            }
        }
        return (*(it->second.func))(call_args, frame);
    }

    return PyValue::None();
}

// -------------------------------------------------
// exec(code)
// -------------------------------------------------
PyValue py_exec(std::vector<PyValue> args, PyFrame& frame){
    if(args.empty() || args[0].type!=PyValue::STR)
        return PyValue::None();

    std::istringstream ss(*args[0].s);
    std::string line;
    PyValue last=PyValue::None();

    while(std::getline(ss,line)){
        if(line.find('=')!=std::string::npos){
            auto p=line.find('=');
            std::string name=line.substr(0,p);
            std::string rhs=line.substr(p+1);
            name.erase(remove_if(name.begin(),name.end(),isspace),name.end());
            last=py_eval({PyValue::Str(rhs)},frame);
            frame.globals[name]=last;
        } else {
            last=py_eval({PyValue::Str(line)},frame);
        }
    }
    return last;
}
