// --- Exception-Mechanismus -----------------------------------------

#[derive(Clone)]
pub struct PyException {
    pub name: String,
    pub message: String,
}

impl PyException {
    pub fn new(name: &str, message: &str) -> Self {
        Self {
            name: name.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone)]
pub enum PyResult {
    Value(PyValue),
    Raise(PyException),
    Break,
    Continue,
    Return(PyValue),
}

// --- Helfer --------------------------------------------------------

pub fn raise(name: &str, msg: &str) -> PyResult {
    PyResult::Raise(PyException::new(name, msg))
}

pub fn ok(v: PyValue) -> PyResult {
    PyResult::Value(v)
}

// --- try / except / finally ---------------------------------------

pub fn py_try<F, H, G>(
    try_block: F,
    except_block: Option<H>,
    finally_block: Option<G>,
) -> PyResult
where
    F: Fn() -> PyResult,
    H: Fn(PyException) -> PyResult,
    G: Fn() -> PyResult,
{
    let mut result = match try_block() {
        PyResult::Raise(e) => {
            if let Some(handler) = except_block {
                handler(e)
            } else {
                PyResult::Raise(e)
            }
        }
        other => other,
    };

    if let Some(finally_fn) = finally_block {
        match finally_fn() {
            PyResult::Raise(e) => return PyResult::Raise(e),
            _ => {}
        }
    }

    result
}

// --- throw als Python-Funktion ------------------------------------

pub fn py_raise(args: Vec<PyValue>) -> PyValue {
    let msg = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => "Exception".into(),
    };
    PyValue::Exception(msg)
}

// --- Vergleich / Bool ---------------------------------------------

pub fn py_truthy(v: &PyValue) -> bool {
    match v {
        PyValue::None => false,
        PyValue::Bool(b) => *b,
        PyValue::Int(i) => *i != 0,
        PyValue::Float(f) => *f != 0.0,
        PyValue::Str(s) => !s.is_empty(),
        PyValue::List(l) => !l.is_empty(),
        PyValue::Dict(d) => !d.is_empty(),
        _ => true,
    }
}

// --- if / while / for (strukturell) -------------------------------

pub fn py_if<F, G>(cond: &PyValue, then_fn: F, else_fn: Option<G>) -> PyResult
where
    F: Fn() -> PyResult,
    G: Fn() -> PyResult,
{
    if py_truthy(cond) {
        then_fn()
    } else if let Some(e) = else_fn {
        e()
    } else {
        ok(PyValue::None)
    }
}

pub fn py_while<F, G>(cond_fn: F, body_fn: G) -> PyResult
where
    F: Fn() -> PyValue,
    G: Fn() -> PyResult,
{
    loop {
        if !py_truthy(&cond_fn()) {
            return ok(PyValue::None);
        }
        match body_fn() {
            PyResult::Break => return ok(PyValue::None),
            PyResult::Continue => continue,
            PyResult::Raise(e) => return PyResult::Raise(e),
            PyResult::Return(v) => return PyResult::Return(v),
            _ => {}
        }
    }
}

pub fn py_for<F>(
    iterable: &PyValue,
    body: F,
) -> PyResult
where
    F: Fn(PyValue) -> PyResult,
{
    match iterable {
        PyValue::List(items) => {
            for v in items {
                match body(v.clone()) {
                    PyResult::Break => break,
                    PyResult::Continue => continue,
                    PyResult::Raise(e) => return PyResult::Raise(e),
                    PyResult::Return(v) => return PyResult::Return(v),
                    _ => {}
                }
            }
            ok(PyValue::None)
        }
        _ => raise("TypeError", "object is not iterable"),
    }
}
