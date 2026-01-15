// ===============================
// meta_runtime.cpp
// ===============================
#include "py_runtime.hpp"

// -------------------------------------------------
// setattr(obj, key, value)
// -------------------------------------------------
PyValue py_setattr(std::vector<PyValue> args, PyFrame&){
    if(args.size()<3) return PyValue::None();
    if(args[0].type!=PyValue::DICT || args[1].type!=PyValue::STR)
        return PyValue::None();

    (*args[0].dict)[*args[1].s]=args[2];
    return PyValue::None();
}

// -------------------------------------------------
// getattr(obj, key, default=None)
// -------------------------------------------------
PyValue py_getattr(std::vector<PyValue> args, PyFrame&){
    if(args.size()<2) return PyValue::None();
    if(args[0].type!=PyValue::DICT || args[1].type!=PyValue::STR)
        return PyValue::None();

    auto it=args[0].dict->find(*args[1].s);
    if(it!=args[0].dict->end())
        return it->second;

    if(args.size()>2)
        return args[2];

    return PyValue::None();
}
