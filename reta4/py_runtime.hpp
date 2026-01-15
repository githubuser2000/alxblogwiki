// ===============================
// py_runtime.hpp  (C++17)
// ===============================
#pragma once
#include <string>
#include <vector>
#include <unordered_map>
#include <memory>
#include <functional>
#include <iostream>

struct PyValue;
struct PyFrame;

using PyFunc = std::function<PyValue(std::vector<PyValue>, PyFrame&)>;

struct PyException {
    std::string name;
    std::string message;
};

struct PyValue {
    enum Type { NONE, INT, BOOL, STR, LIST, DICT, FUNC, EXC } type = NONE;

    long long i{};
    bool b{};
    std::shared_ptr<std::string> s;
    std::shared_ptr<std::vector<PyValue>> list;
    std::shared_ptr<std::unordered_map<std::string, PyValue>> dict;
    std::shared_ptr<PyFunc> func;
    std::shared_ptr<PyException> exc;

    PyValue() = default;
    static PyValue None() { return PyValue(); }
    static PyValue Int(long long v){ PyValue x; x.type=INT; x.i=v; return x; }
    static PyValue Bool(bool v){ PyValue x; x.type=BOOL; x.b=v; return x; }
    static PyValue Str(const std::string& v){ PyValue x; x.type=STR; x.s=std::make_shared<std::string>(v); return x; }
    static PyValue List(){ PyValue x; x.type=LIST; x.list=std::make_shared<std::vector<PyValue>>(); return x; }
    static PyValue Dict(){ PyValue x; x.type=DICT; x.dict=std::make_shared<std::unordered_map<std::string, PyValue>>(); return x; }
    static PyValue Func(PyFunc f){ PyValue x; x.type=FUNC; x.func=std::make_shared<PyFunc>(f); return x; }
    static PyValue Exc(const std::string& n,const std::string& m){
        PyValue x; x.type=EXC; x.exc=std::make_shared<PyException>(PyException{n,m}); return x;
    }
};

struct PyFrame {
    std::unordered_map<std::string, PyValue> globals;
};

// ===============================
// center.cpp
// ===============================
#include "py_runtime.hpp"
#include <fstream>
#include <sstream>
#include <cstdlib>

// ---------- classify placeholder ----------
PyValue py_classify(std::vector<PyValue> args, PyFrame&) {
    return PyValue::None();
}

// ---------- moduloA ----------
PyValue py_moduloA(std::vector<PyValue> args, PyFrame& frame) {
    if(args.empty() || args[0].type!=PyValue::LIST)
        return PyValue::Exc("TypeError","zahlen must be list");

    for(auto& v : *args[0].list){
        if(v.type!=PyValue::INT) continue;
        long long n=v.i;
        for(int var=2;var<26;var++){
            std::cout<<n<<" % "<<var<<" = ";
            long long mod=n%var;
            std::cout<<mod;
            auto it=frame.globals.find("classify");
            if(it!=frame.globals.end() && it->second.type==PyValue::FUNC){
                (*(it->second.func))({PyValue::Int(mod)},frame);
            }
            std::cout<<", ";
            mod=var-mod;
            std::cout<<mod;
            if(it!=frame.globals.end() && it->second.type==PyValue::FUNC){
                (*(it->second.func))({PyValue::Int(mod)},frame);
            }
            std::cout<<"\n";
        }
    }
    return PyValue::None();
}

// ---------- CSV parsing ----------
std::vector<std::string> parse_csv(const std::string& line,char sep){
    std::vector<std::string> out;
    std::string buf;
    bool in=false;
    for(size_t i=0;i<line.size();++i){
        char c=line[i];
        if(c=='"'){ in=!in; }
        else if(c==sep && !in){ out.push_back(buf); buf.clear(); }
        else buf.push_back(c);
    }
    out.push_back(buf);
    return out;
}

PyValue py_read_csv(std::vector<PyValue> args, PyFrame&){
    if(args.empty()||args[0].type!=PyValue::STR) return PyValue::None();
    char sep=',';
    if(args.size()>1 && args[1].type==PyValue::STR && !args[1].s->empty())
        sep=(*args[1].s)[0];

    std::ifstream f(*args[0].s);
    if(!f) return PyValue::Exc("IOError","cannot open file");

    std::string line;
    std::getline(f,line);
    auto headers=parse_csv(line,sep);

    PyValue table=PyValue::List();
    while(std::getline(f,line)){
        auto cols=parse_csv(line,sep);
        PyValue row=PyValue::Dict();
        for(size_t i=0;i<headers.size();++i){
            std::string v = (i<cols.size()?cols[i]:"");
            (*row.dict)[headers[i]]=PyValue::Str(v);
        }
        table.list->push_back(row);
    }
    return table;
}

// ---------- shell ----------
PyValue py_shell(std::vector<PyValue> args, PyFrame&){
    if(args.empty()||args[0].type!=PyValue::STR) return PyValue::Int(0);
    int rc=std::system(args[0].s->c_str());
    return PyValue::Int(rc);
}

// ---------- init ----------
int main(){
    PyFrame frame;
    frame.globals["classify"]=PyValue::Func(py_classify);
    frame.globals["moduloA"]=PyValue::Func(py_moduloA);
    frame.globals["read_csv"]=PyValue::Func(py_read_csv);
    frame.globals["shell"]=PyValue::Func(py_shell);

    // Beispielaufruf wie Python:
    PyValue nums=PyValue::List();
    nums.list->push_back(PyValue::Int(7));
    frame.globals["moduloA"].func->operator()({nums},frame);

    return 0;
}
