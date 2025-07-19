#[derive(Debug)]
pub enum ErrT {
    NotImplemented,
    TarUnarchive,
}

impl ErrT {
    fn explain(&self) -> Option<&'static str> {
        match self {
            Self::NotImplemented => {
                Some("This feature has not been implemented yet.")
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct StackFrame {
    err: ErrT,
    ctx: Option<String>,
}

#[derive(Debug)]
pub struct ErrStack {
    stack: Vec<StackFrame>,
}

impl std::error::Error for ErrStack {}

impl ErrStack {
    pub fn new(err_t: ErrT) -> Self {
        Self {
            stack: vec![StackFrame {
                err: err_t,
                ctx: None,
            }],
        }
    }
    pub fn push(mut self, err: ErrT) -> Self {
        self.stack.push(StackFrame { err, ctx: None });
        self
    }
    pub fn ctx(mut self, ctx: String) -> Self {
        if let Some(last) = self.stack.last_mut() {
            last.ctx = Some(ctx);
        }
        self
    }
}
impl std::fmt::Display for ErrStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Oops! One or more errors occurred;")?;
        for (indent, item) in self.stack.iter().enumerate() {
            let indent = "  ".repeat(indent + 1);
            let err_t = &item.err;
            let ctx = item.ctx.as_ref();
            if let Some(ctx) = ctx {
                writeln!(f, "{indent}{err_t:?} :: {ctx}")?;
            } else if let Some(blanket_explanation) = item.err.explain() {
                writeln!(f, "{indent}{err_t:?} :: {blanket_explanation}")?;
            } else {
                writeln!(f, "{indent}{err_t:?} :: details not available")?;
            }
        }
        Ok(())
    }
}
