use std::cell::RefCell;
use std::rc::Rc;

use crate::py_runtime::*;

/* =========================================================
 * Globale i18n-State (exakt wie Python-Modul)
 * ========================================================= */

thread_local! {
    static CURRENT_LANG: RefCell<String> = RefCell::new("en".to_string());
    static WORDS: RefCell<PyDict> = RefCell::new(PyDict::new());
}

/* =========================================================
 * set_lang(lang)
 * ========================================================= */

fn py_set_lang(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    if let Some(PyValue::Str(lang)) = args.get(0) {
        CURRENT_LANG.with(|l| {
            *l.borrow_mut() = lang.clone();
        });
    }
    Ok(PyValue::None)
}

/* =========================================================
 * add_words(lang, dict)
 * ========================================================= */

fn py_add_words(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    if args.len() != 2 {
        return Ok(PyValue::None);
    }

    let lang = match &args[0] {
        PyValue::Str(s) => s.clone(),
        _ => return Ok(PyValue::None),
    };

    let dict = match &args[1] {
        PyValue::Dict(d) => d.clone(),
        _ => return Ok(PyValue::None),
    };

    WORDS.with(|w| {
        w.borrow_mut().set(
            lang.into(),
            PyValue::Dict(dict),
        );
    });

    Ok(PyValue::None)
}

/* =========================================================
 * get(key)
 * ========================================================= */

fn py_get(
    args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let key = match args.get(0) {
        Some(PyValue::Str(s)) => s.clone(),
        _ => return Ok(PyValue::None),
    };

    let lang = CURRENT_LANG.with(|l| l.borrow().clone());

    WORDS.with(|w| {
        if let Some(PyValue::Dict(lang_dict)) = w.borrow().get(&lang) {
            if let Some(v) = lang_dict.borrow().get(&key) {
                return Ok(v.clone());
            }
        }
        // Python-Fallback: key selbst zurückgeben
        Ok(PyValue::Str(key))
    })
}

/* =========================================================
 * get_lang()
 * ========================================================= */

fn py_get_lang(
    _args: Vec<PyValue>,
    _frame: &mut PyFrame,
) -> Result<PyValue, PyValue> {
    let lang = CURRENT_LANG.with(|l| l.borrow().clone());
    Ok(PyValue::Str(lang))
}

/* =========================================================
 * Modul-Initialisierung i18n.words
 * ========================================================= */

pub fn init_i18n_words_module(frame: &mut PyFrame) {
    let mut g = frame.globals.borrow_mut();

    g.set("set_lang".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "set_lang".into(),
            func: py_set_lang,
        }))
    );

    g.set("get_lang".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "get_lang".into(),
            func: py_get_lang,
        }))
    );

    g.set("add_words".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "add_words".into(),
            func: py_add_words,
        }))
    );

    g.set("get".into(),
        PyValue::Function(Rc::new(PyFunction {
            name: "get".into(),
            func: py_get,
        }))
    );
}
