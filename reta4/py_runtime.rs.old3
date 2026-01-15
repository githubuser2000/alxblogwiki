// --- Klassen & Objekte ---------------------------------------------

#[derive(Clone)]
pub struct PyClass {
    pub name: String,
    pub dict: Rc<RefCell<BTreeMap<String, PyValue>>>,
}

#[derive(Clone)]
pub struct PyObject {
    pub class: Rc<PyClass>,
    pub fields: Rc<RefCell<BTreeMap<String, PyValue>>>,
}

impl PyClass {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            dict: Rc::new(RefCell::new(BTreeMap::new())),
        })
    }

    pub fn set_attr(&self, name: &str, val: PyValue) {
        self.dict.borrow_mut().insert(name.into(), val);
    }

    pub fn get_attr(&self, name: &str) -> Option<PyValue> {
        self.dict.borrow().get(name).cloned()
    }
}

impl PyObject {
    pub fn new(class: Rc<PyClass>) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn set_attr(&self, name: &str, val: PyValue) {
        self.fields.borrow_mut().insert(name.into(), val);
    }

    pub fn get_attr(&self, name: &str) -> Option<PyValue> {
        if let Some(v) = self.fields.borrow().get(name) {
            return Some(v.clone());
        }
        self.class.get_attr(name)
    }
}

// --- Erweiterung PyValue -------------------------------------------

impl PyValue {
    pub fn as_object(&self) -> Option<PyObject> {
        match self {
            PyValue::Dict(_) => None,
            PyValue::Function(_) => None,
            PyValue::Exception(_) => None,
            _ => None,
        }
    }
}

// --- Methodenbindung (self / cls) ----------------------------------

pub fn bind_method(
    func: PyValue,
    target: PyValue,
) -> PyValue {
    match func {
        PyValue::Function(f) => {
            PyValue::Function(Rc::new(move |mut args| {
                let mut new_args = vec![target.clone()];
                new_args.append(&mut args);
                f(new_args)
            }))
        }
        _ => PyValue::Exception("TypeError: not callable".into()),
    }
}

// --- getattr / setattr ---------------------------------------------

pub fn py_getattr(obj: &PyObject, name: &str) -> PyValue {
    if let Some(v) = obj.get_attr(name) {
        match v {
            PyValue::Function(_) => bind_method(v, PyValue::Dict(obj.fields.borrow().clone())),
            _ => v,
        }
    } else {
        PyValue::Exception(format!(
            "AttributeError: '{}' has no attribute '{}'",
            obj.class.name, name
        ))
    }
}

pub fn py_setattr(obj: &PyObject, name: &str, val: PyValue) {
    obj.set_attr(name, val);
}

// --- Enum-Support (wie Python Enum) --------------------------------

pub fn py_enum(name: &str, members: &[&str]) -> Rc<PyClass> {
    let cls = PyClass::new(name);
    for (i, m) in members.iter().enumerate() {
        cls.set_attr(
            m,
            PyValue::Int(i as i64),
        );
    }
    cls
}
