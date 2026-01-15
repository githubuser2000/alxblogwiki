// ===============================
// register_all.cpp
// ===============================
#include "py_runtime.hpp"

// forward decls
PyValue py_where(std::vector<PyValue>,PyFrame&);
PyValue py_select(std::vector<PyValue>,PyFrame&);
PyValue py_join(std::vector<PyValue>,PyFrame&);
PyValue py_ansi(std::vector<PyValue>,PyFrame&);
PyValue py_set_lang(std::vector<PyValue>,PyFrame&);
PyValue py_add_words(std::vector<PyValue>,PyFrame&);
PyValue py_get_word(std::vector<PyValue>,PyFrame&);

void register_all(PyFrame& frame){
    frame.globals["where"]     = PyValue::Func(py_where);
    frame.globals["select"]   = PyValue::Func(py_select);
    frame.globals["join"]     = PyValue::Func(py_join);
    frame.globals["ansi"]     = PyValue::Func(py_ansi);
    frame.globals["set_lang"] = PyValue::Func(py_set_lang);
    frame.globals["add_words"]= PyValue::Func(py_add_words);
    frame.globals["get"]      = PyValue::Func(py_get_word);
}
