use std::cell::RefCell;
use std::rc::Rc;

use crate::py_runtime::*;

/* =========================================================
 * CSV-Zeile parsen (Python-ähnlich, permissiv)
 * ========================================================= */

fn parse_csv_line(line: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_string = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_string && chars.peek() == Some(&'"') {
                    buf.push('"');
                    chars.next();
                } else {
                    in_string = !in_string;
                }
            }
            _ if c == sep && !in_string => {
                out.push(buf.clone());
                buf.clear();
            }
            _ => buf.push(c),
        }
    }

    out.push(buf);
    out
}

/* =========================================================
 * Python: read_csv(path, sep=',')
 * ========================================================= */

fn py_read_csv(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let path = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => return Ok(PyValue::None),
    };

    let sep = match args.get(1) {
        Some(PyValue::Str(s)) => s.chars().next().unwrap_or(','),
        _ => ',',
    };

    let content = std::fs::read_to_string(&path)
        .map_err(|e| PyValue::Exception(Rc::new(PyException {
            name: "IOError".into(),
            message: e.to_string(),
        })))?;

    let mut lines = content.lines();
    let header_line = match lines.next() {
        Some(l) => l,
        None => return Ok(PyValue::List(Rc::new(RefCell::new(vec![])))),
    };

    let headers = parse_csv_line(header_line, sep);

    let table = Rc::new(RefCell::new(Vec::<PyValue>::new()));

    for line in lines {
        let cols = parse_csv_line(line, sep);
        let row = PyDict::new();

        for (i, h) in headers.iter().enumerate() {
            let v = cols.get(i).cloned().unwrap_or_default();

            // JSON-in-CSV (sehr permissiv)
            let pyv = if v.starts_with('[') && v.ends_with(']') {
                PyValue::Str(v) // später eval()-artig verarbeitet
            } else {
                PyValue::Str(v)
            };

            row.set(h.clone(), pyv);
        }

        table.borrow_mut().push(PyValue::Dict(Rc::new(RefCell::new(row))));
    }

    Ok(PyValue::List(table))
}

/* =========================================================
 * Python: select(table, keys)
 * ========================================================= */

fn py_select(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let table = match args.get(0) {
        Some(PyValue::List(l)) => l.clone(),
        _ => return Ok(PyValue::None),
    };

    let keys = match args.get(1) {
        Some(PyValue::List(k)) => k.borrow().clone(),
        _ => return Ok(PyValue::None),
    };

    let out = Rc::new(RefCell::new(Vec::new()));

    for row in table.borrow().iter() {
        if let PyValue::Dict(d) = row {
            let mut new = PyDict::new();
            for k in keys.iter() {
                if let PyValue::Str(ks) = k {
                    if let Some(v) = d.borrow().get(ks) {
                        new.set(ks.clone(), v.clone());
                    }
                }
            }
            out.borrow_mut().push(PyValue::Dict(Rc::new(RefCell::new(new))));
        }
    }

    Ok(PyValue::List(out))
}

/* =========================================================
 * Python: where(table, func)
 * ========================================================= */

fn py_where(
    args: Vec<PyValue>,
    frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let table = match args.get(0) {
        Some(PyValue::List(l)) => l.clone(),
        _ => return Ok(PyValue::None),
    };

    let func = match args.get(1) {
        Some(PyValue::Function(f)) => f.clone(),
        _ => return Ok(PyValue::None),
    };

    let out = Rc::new(RefCell::new(Vec::new()));

    for row in table.borrow().iter() {
        let res = func.call(vec![row.clone()], frame)?;
        if matches!(res, PyValue::Bool(true)) {
            out.borrow_mut().push(row.clone());
        }
    }

    Ok(PyValue::List(out))
}

/* =========================================================
 * Python: join(left, right, key)
 * ========================================================= */

fn py_join(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let left = match args.get(0) {
        Some(PyValue::List(l)) => l.clone(),
        _ => return Ok(PyValue::None),
    };

    let right = match args.get(1) {
        Some(PyValue::List(l)) => l.clone(),
        _ => return Ok(PyValue::None),
    };

    let key = match args.get(2) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => return Ok(PyValue::None),
    };

    let out = Rc::new(RefCell::new(Vec::new()));

    for lrow in left.borrow().iter() {
        for rrow in right.borrow().iter() {
            if let (PyValue::Dict(ld), PyValue::Dict(rd)) = (lrow, rrow) {
                let lv = ld.borrow().get(&key);
                let rv = rd.borrow().get(&key);

                if lv.is_some() && lv == rv {
                    let mut merged = PyDict::new();

                    for (k, v) in ld.borrow().iter() {
                        merged.set(k.clone(), v.clone());
                    }
                    for (k, v) in rd.borrow().iter() {
                        merged.set(k.clone(), v.clone());
                    }

                    out.borrow_mut().push(
                        PyValue::Dict(Rc::new(RefCell::new(merged)))
                    );
                }
            }
        }
    }

    Ok(PyValue::List(out))
}

/* =========================================================
 * Modul-Init
 * ========================================================= */

pub fn init_csv_module(frame: &mut PyFrame) {
    let mut g = frame.globals.borrow_mut();

    macro_rules! f {
        ($n:expr, $f:expr) => {
            g.set($n.into(),
                PyValue::Function(Rc::new(PyFunction {
                    name: $n.into(),
                    func: $f,
                }))
            );
        }
    }

    f!("read_csv", py_read_csv);
    f!("select", py_select);
    f!("where", py_where);
    f!("join", py_join);
}
