// --- Expression AST ------------------------------------------------

#[derive(Clone, Debug)]
pub enum Expr {
    Value(PyValue),
    Var(String),
    BinOp(Box<Expr>, String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
    Assign(String, Expr),
    Return(Expr),
}

// --- Eval Context --------------------------------------------------

#[derive(Clone)]
pub struct EvalCtx {
    pub locals: BTreeMap<String, PyValue>,
    pub globals: BTreeMap<String, PyValue>,
}

impl EvalCtx {
    pub fn new(globals: BTreeMap<String, PyValue>) -> Self {
        Self {
            locals: BTreeMap::new(),
            globals,
        }
    }

    pub fn get(&self, name: &str) -> Option<PyValue> {
        self.locals.get(name)
            .or_else(|| self.globals.get(name))
            .cloned()
    }

    pub fn set(&mut self, name: &str, val: PyValue) {
        self.locals.insert(name.into(), val);
    }
}

// --- Expression Evaluation ----------------------------------------

pub fn eval_expr(expr: &Expr, ctx: &mut EvalCtx) -> PyResult {
    match expr {
        Expr::Value(v) => ok(v.clone()),

        Expr::Var(name) => match ctx.get(name) {
            Some(v) => ok(v),
            None => raise("NameError", &format!("name '{}' is not defined", name)),
        },

        Expr::BinOp(a, op, b) => {
            let va = match eval_expr(a, ctx)? {
                PyResult::Value(v) => v,
                x => return x,
            };
            let vb = match eval_expr(b, ctx)? {
                PyResult::Value(v) => v,
                x => return x,
            };

            match (va, vb, op.as_str()) {
                (PyValue::Int(x), PyValue::Int(y), "+") => ok(PyValue::Int(x + y)),
                (PyValue::Int(x), PyValue::Int(y), "-") => ok(PyValue::Int(x - y)),
                (PyValue::Int(x), PyValue::Int(y), "*") => ok(PyValue::Int(x * y)),
                (PyValue::Int(x), PyValue::Int(y), "/") => ok(PyValue::Float(x as f64 / y as f64)),
                (PyValue::Str(a), PyValue::Str(b), "+") => ok(PyValue::Str(a + &b)),
                _ => raise("TypeError", "unsupported operand type"),
            }
        }

        Expr::Call(func, args) => {
            let f = match eval_expr(func, ctx)? {
                PyResult::Value(PyValue::Function(f)) => f,
                _ => return raise("TypeError", "object is not callable"),
            };

            let mut vals = Vec::new();
            for a in args {
                match eval_expr(a, ctx)? {
                    PyResult::Value(v) => vals.push(v),
                    x => return x,
                }
            }
            ok(f(vals))
        }
    }
}

// --- Statement Execution ------------------------------------------

pub fn exec_stmt(stmt: &Stmt, ctx: &mut EvalCtx) -> PyResult {
    match stmt {
        Stmt::Expr(e) => eval_expr(e, ctx),
        Stmt::Assign(name, e) => {
            match eval_expr(e, ctx)? {
                PyResult::Value(v) => {
                    ctx.set(name, v);
                    ok(PyValue::None)
                }
                x => x,
            }
        }
        Stmt::Return(e) => match eval_expr(e, ctx)? {
            PyResult::Value(v) => PyResult::Return(v),
            x => x,
        },
    }
}

// --- eval / exec ---------------------------------------------------

pub fn py_eval(expr: Expr, globals: BTreeMap<String, PyValue>) -> PyResult {
    let mut ctx = EvalCtx::new(globals);
    eval_expr(&expr, &mut ctx)
}

pub fn py_exec(stmts: Vec<Stmt>, globals: BTreeMap<String, PyValue>) -> PyResult {
    let mut ctx = EvalCtx::new(globals);
    for s in stmts {
        match exec_stmt(&s, &mut ctx) {
            PyResult::Return(v) => return ok(v),
            PyResult::Raise(e) => return PyResult::Raise(e),
            _ => {}
        }
    }
    ok(PyValue::None)
}
