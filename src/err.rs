//! Error handling

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum Oops {
    Placeholder,
}

impl Oops {
    /// In some cases, there might only be one possible explanation for an
    /// error type, in which case we can centralize those explanations here
    /// instead of needing to use [Error::because] all over the place.
    pub fn explain(&self) -> Option<&'static str> {
        match self {
            Self::Placeholder => {
                Some("This is a placeholder. If it shows up in prod, someone forgot to finish what they were doing.")
            }
        }
    }
}

#[derive(Debug)]
struct Oopsie {
    variant: Oops,
    ctx: Option<String>,
}

#[derive(Debug)]
pub struct Error {
    /// A series of unfortunate events, from first to last.
    ///
    /// Almost like a stack of errors 🤔
    oopsies: Vec<Oopsie>,
    http_status: StatusCode,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            oopsies: Vec::new(),
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// An adequate and simple error framework. Start by creating an error;
///
/// ```
/// // Start by making a new error.
/// let e = Error::default()
/// // Then, identify what went wrong.
/// e.wrap(Oops::OpenAIKeyMissing);
/// // Optionally, say why.
/// fn bad_stuff() {
///     e.wrap(Oops::OpenAIChatResponse).because(format!(
///         "In function {}, we encountered {}",
///         type_name(bad_stuff),
///         "some other error type"
///     ))
/// }
/// ```
///
/// As errors flow up through a call stack, receivers can call [Self::wrap]
/// and/or [Self::because] to add context to the error.
impl Error {
    /// Append an error-type to the stack.
    pub fn wrap(mut self, oops: Oops) -> Self {
        self.oopsies.push(Oopsie {
            variant: oops,
            ctx: None,
        });
        self
    }
    /// Add a description by mutating the most recent entry on the error stack.
    /// We expect this to be called immediately after a call to [Self::wrap]
    /// to enhance the error-type with details from the context where the
    /// error happened.
    pub fn because(mut self, ctx: String) -> Self {
        if let Some(last) = self.oopsies.last_mut() {
            last.ctx = Some(ctx);
        }
        self
    }
    /// Cause the error to present to the client as a bad request
    pub fn bad_request(mut self) -> Self {
        self.http_status = StatusCode::BAD_REQUEST;
        self
    }
}

impl<T: std::error::Error> From<T> for Error {
    fn from(value: T) -> Self {
        Self::default()
            .wrap(Oops::Placeholder)
            .because(format!("outside error: {value:?}"))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Oops! One or more errors occurred;")?;
        let alt = "details not available";
        for (indent, item) in self.oopsies.iter().enumerate() {
            let indent = "  ".repeat(indent + 1);
            let er_code = &item.variant;
            let ctx = item.ctx.as_ref();
            if let Some(ctx) = ctx {
                writeln!(f, "{indent}{er_code:?} :: {ctx}")?;
            } else if let Some(exp) = er_code.explain() {
                writeln!(f, "{indent}{er_code:?} :: {exp}")?;
            } else {
                writeln!(f, "{indent}{er_code:?} :: {alt}")?;
            }
        }
        Ok(())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::info!("{self}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            .into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
