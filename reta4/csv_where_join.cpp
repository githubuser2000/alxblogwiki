// ===============================
// csv_where_join.cpp  (C++17)
// ===============================
#include "py_runtime.hpp"

// ---------- where(table, func) ----------
PyValue py_where(std::vector<PyValue> args, PyFrame& frame){
    if(args.size()<2) return PyValue::None();
    if(args[0].type!=PyValue::LIST || args[1].type!=PyValue::FUNC)
        return PyValue::None();

    PyValue out = PyValue::List();
    for(auto& row : *args[0].list){
        PyValue res = (*(args[1].func))({row}, frame);
        if(res.type==PyValue::BOOL && res.b){
            out.list->push_back(row);
        }
    }
    return out;
}

// ---------- select(table, keys) ----------
PyValue py_select(std::vector<PyValue> args, PyFrame&){
    if(args.size()<2) return PyValue::None();
    if(args[0].type!=PyValue::LIST || args[1].type!=PyValue::LIST)
        return PyValue::None();

    PyValue out = PyValue::List();
    for(auto& row : *args[0].list){
        if(row.type!=PyValue::DICT) continue;
        PyValue n = PyValue::Dict();
        for(auto& k : *args[1].list){
            if(k.type==PyValue::STR){
                auto it = row.dict->find(*k.s);
                if(it!=row.dict->end()){
                    (*n.dict)[*k.s] = it->second;
                }
            }
        }
        out.list->push_back(n);
    }
    return out;
}

// ---------- join(left, right, key) ----------
PyValue py_join(std::vector<PyValue> args, PyFrame&){
    if(args.size()<3) return PyValue::None();
    if(args[0].type!=PyValue::LIST ||
       args[1].type!=PyValue::LIST ||
       args[2].type!=PyValue::STR)
        return PyValue::None();

    std::string key = *args[2].s;
    PyValue out = PyValue::List();

    for(auto& l : *args[0].list){
        if(l.type!=PyValue::DICT) continue;
        auto itl = l.dict->find(key);
        if(itl==l.dict->end()) continue;

        for(auto& r : *args[1].list){
            if(r.type!=PyValue::DICT) continue;
            auto itr = r.dict->find(key);
            if(itr==r.dict->end()) continue;

            if(itl->second.type==itr->second.type &&
               itl->second.type==PyValue::STR &&
               *itl->second.s == *itr->second.s){

                PyValue merged = PyValue::Dict();
                for(auto& kv : *l.dict) (*merged.dict)[kv.first]=kv.second;
                for(auto& kv : *r.dict) (*merged.dict)[kv.first]=kv.second;
                out.list->push_back(merged);
            }
        }
    }
    return out;
}
