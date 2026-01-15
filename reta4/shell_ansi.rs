use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

use crate::py_runtime::*;

/* =========================================================
 * ANSI-Codes (wie Python-Dicts)
 * ========================================================= */

thread_local! {
    static ANSI_COLORS: RefCell<PyDict> = RefCell::new({
        let d = PyDict::new();
        d.set("black".into(),   PyValue::Str("30".into()));
        d.set("red".into(),     PyValue::Str("31".into()));
        d.set("green".into(),   PyValue::Str("32".into()));
        d.set("yellow".into(),  PyValue::Str("33".into()));
        d.set("blue".into(),    PyValue::Str("34".into()));
        d.set("magenta".into(), PyValue::Str("35".into()));
        d.set("cyan".into(),    PyValue::Str("36".into()));
        d.set("white".into(),   PyValue::Str("37".into()));
        d
    });
}

/* =========================================================
 * ansi(text, color=None, bold=False, bg=None)
 * ========================================================= */

fn py_ansi(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let text = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => "".into(),
    };

    let color = args.get(1).cloned();
    let bold = matches!(args.get(2), Some(PyValue::Bool(true)));
    let bg = args.get(3).cloned();

    let mut codes: Vec<String> = Vec::new();

    if bold {
        codes.push("1".into());
    }

    if let Some(PyValue::Str(c)) = color {
        ANSI_COLORS.with(|m| {
            if let Some(PyValue::Str(code)) = m.borrow().get(&c) {
                codes.push(code.clone());
            }
        });
    }

    if let Some(PyValue::Str(c)) = bg {
        ANSI_COLORS.with(|m| {
            if let Some(PyValue::Str(code)) = m.borrow().get(&c) {
                // Hintergrund = Vordergrund + 10
                if let Ok(n) = code.parse::<i32>() {
                    codes.push((n + 10).to_string());
                }
            }
        });
    }

    if codes.is_empty() {
        return Ok(PyValue::Str(text));
    }

    let seq = format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text);
    Ok(PyValue::Str(seq))
}

/* =========================================================
 * terminal_width()
 * ========================================================= */

fn py_terminal_width(
    _args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(n) = cols.parse::<i64>() {
            return Ok(PyValue::Int(n));
        }
    }

    // Fallback: stty
    if let Ok(out) = Command::new("sh")
        .arg("-c")
        .arg("stty size 2>/dev/null | awk '{print $2}'")
        .output()
    {
        if let Ok(s) = String::from_utf8(out.stdout) {
            if let Ok(n) = s.trim().parse::<i64>() {
                return Ok(PyValue::Int(n));
            }
        }
    }

    Ok(PyValue::Int(80))
}

/* =========================================================
 * getTextWrapThings(text, width=None)
 * ========================================================= */

fn py_getTextWrapThings(
    args: Vec<PyValue>,
    frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let text = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => "".into(),
    };

    let width = match args.get(1) {
        Some(PyValue::Int(i)) => *i as usize,
        _ => {
            if let PyValue::Int(i) = py_terminal_width(vec![], frame)? {
                i as usize
            } else {
                80
            }
        }
    };

    let mut out = Vec::new();
    let mut line = String::new();

    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 > width {
            out.push(PyValue::Str(line.clone()));
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }

    if !line.is_empty() {
        out.push(PyValue::Str(line));
    }

    Ok(PyValue::List(Rc::new(RefCell::new(out))))
}

/* =========================================================
 * shell(cmd)  -- wie os.system / subprocess.call
 * ========================================================= */

fn py_shell(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let cmd = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => return Ok(PyValue::Int(0)),
    };

    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status();

    match status {
        Ok(s) => Ok(PyValue::Int(s.code().unwrap_or(0) as i64)),
        Err(_) => Ok(PyValue::Int(-1)),
    }
}

/* =========================================================
 * Modul-Init shell/ansi
 * ========================================================= */

pub fn init_shell_module(frame: &mut PyFrame) {
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

    f!("ansi", py_ansi);
    f!("terminal_width", py_terminal_width);
    f!("getTextWrapThings", py_getTextWrapThings);
    f!("shell", py_shell);
}
