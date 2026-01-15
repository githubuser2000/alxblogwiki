// ===============================
// final_register.cpp
// ===============================
#include "py_runtime.hpp"

// forward decls
PyValue py_eval(std::vector<PyValue>,PyFrame&);
PyValue py_exec(std::vector<PyValue>,PyFrame&);
PyValue py_setattr(std::vector<PyValue>,PyFrame&);
PyValue py_getattr(std::vector<PyValue>,PyFrame&);

void register_meta(PyFrame& frame){
    frame.globals["eval"]    = PyValue::Func(py_eval);
    frame.globals["exec"]    = PyValue::Func(py_exec);
    frame.globals["setattr"] = PyValue::Func(py_setattr);
    frame.globals["getattr"] = PyValue::Func(py_getattr);
}
