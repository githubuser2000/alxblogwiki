use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/* ============================
 * Python-Wert (dynamisch!)
 * ============================ */

#[derive(Clone)]
pub enum PyValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Rc<RefCell<Vec<PyValue>>>),
    Dict(Rc<RefCell<PyDict>>),
    Function(Rc<PyFunction>),
    Exception(Rc<PyException>),
}

impl fmt::Debug for PyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyValue::None => write!(f, "None"),
            PyValue::Bool(b) => write!(f, "{}", b),
            PyValue::Int(i) => write!(f, "{}", i),
            PyValue::Float(fl) => write!(f, "{}", fl),
            PyValue::Str(s) => write!(f, "\"{}\"", s),
            PyValue::List(_) => write!(f, "<list>"),
            PyValue::Dict(_) => write!(f, "<dict>"),
            PyValue::Function(_) => write!(f, "<function>"),
            PyValue::Exception(e) => write!(f, "<exception {:?}>", e),
        }
    }
}

/* ============================
 * Truthiness exakt wie Python
 * ============================ */

impl PyValue {
    pub fn truthy(&self) -> bool {
        match self {
            PyValue::None => false,
            PyValue::Bool(b) => *b,
            PyValue::Int(i) => *i != 0,
            PyValue::Float(f) => *f != 0.0,
            PyValue::Str(s) => !s.is_empty(),
            PyValue::List(l) => !l.borrow().is_empty(),
            PyValue::Dict(d) => !d.borrow().is_empty(),
            PyValue::Function(_) => true,
            PyValue::Exception(_) => true,
        }
    }
}

/* ============================
 * Ordered Dict (Insertion!)
 * ============================ */

#[derive(Clone)]
pub struct PyDict {
    keys: Vec<String>,
    values: HashMap<String, PyValue>,
}

impl PyDict {
    pub fn new() -> Self {
        PyDict {
            keys: Vec::new(),
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: PyValue) {
        if !self.values.contains_key(&key) {
            self.keys.push(key.clone());
        }
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> PyValue {
        self.values.get(key).cloned().unwrap_or(PyValue::None)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn iter(&self) -> Vec<(String, PyValue)> {
        self.keys
            .iter()
            .map(|k| (k.clone(), self.values.get(k).unwrap().clone()))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

/* ============================
 * Exceptions (als Werte!)
 * ============================ */

#[derive(Clone)]
pub struct PyException {
    pub name: String,
    pub message: String,
}

impl fmt::Debug for PyException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

/* ============================
 * Stack Frame
 * ============================ */

#[derive(Clone)]
pub struct PyFrame {
    pub locals: Rc<RefCell<PyDict>>,
    pub globals: Rc<RefCell<PyDict>>,
}

/* ============================
 * Python-Funktion
 * ============================ */

pub struct PyFunction {
    pub name: String,
    pub func: fn(Vec<PyValue>, &mut PyFrame) -> Result<PyValue, PyValue>,
}

impl PyFunction {
    pub fn call(
        &self,
        args: Vec<PyValue>,
        frame: &mut PyFrame,
    ) -> Result<PyValue, PyValue> {
        (self.func)(args, frame)
    }
}

/* ============================
 * Beispiel: primCreativity
 * (Dummy – wird später exakt
 * aus Python rekonstruiert)
 * ============================ */

fn prim_creativity(args: Vec<PyValue>, _frame: &mut PyFrame) -> Result<PyValue, PyValue> {
    if args.len() != 1 {
        return Err(PyValue::Exception(Rc::new(PyException {
            name: "TypeError".to_string(),
            message: "primCreativity expects 1 argument".to_string(),
        })));
    }

    match &args[0] {
        PyValue::Int(n) => {
            if *n < 2 {
                Ok(PyValue::Bool(false))
            } else {
                for i in 2..*n {
                    if n % i == 0 {
                        return Ok(PyValue::Bool(false));
                    }
                }
                Ok(PyValue::Bool(true))
            }
        }
        _ => Err(PyValue::Exception(Rc::new(PyException {
            name: "TypeError".to_string(),
            message: "expected int".to_string(),
        }))),
    }
}

/* ============================
 * Test-Harness (Python-Stil)
 * ============================ */

fn main() {
    let globals = Rc::new(RefCell::new(PyDict::new()));
    let locals = Rc::new(RefCell::new(PyDict::new()));

    let mut frame = PyFrame { globals, locals };

    let prim_fn = PyFunction {
        name: "primCreativity".to_string(),
        func: prim_creativity,
    };

    let fval = PyValue::Function(Rc::new(prim_fn));

    let result = match fval {
        PyValue::Function(f) => f.call(vec![PyValue::Int(17)], &mut frame),
        _ => unreachable!(),
    };

    println!("Result = {:?}", result);
}
