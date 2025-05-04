use std::any::Any;

pub struct PanicData {
    // TODO: does this need to be an `Option`?
    pub payload: Box<dyn Any + 'static>,
    pub location: Option<PanicLocation>,
}

impl PanicData {
    /// Attempts to convert the panic payload to a string (either [`&str`](str)
    /// or [`String`]), returning [`None`] if the payload was neither `&str` nor
    /// `String`.
    pub fn payload_as_string(&self) -> Option<&str> {
        // Try downcasting to a &str
        if let Some(s) = self.payload.downcast_ref::<&str>() {
            return Some(s);
        }

        // Downcasting to a &str failed, try downcasting to a String
        if let Some(s) = self.payload.downcast_ref::<String>() {
            return Some(s);
        }

        // Downcasting to a String failed, give up and return None
        None
    }
}

impl std::fmt::Debug for PanicData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanicData")
            .field(
                "payload",
                match self.payload_as_string().as_ref() {
                    Some(msg) => msg,
                    None => &self.payload,
                },
            )
            .field("location", &self.location)
            .finish()
    }
}

impl From<chillpill::PanicData> for PanicData {
    fn from(value: chillpill::PanicData) -> Self {
        Self {
            payload: value.payload,
            location: value.location.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PanicLocation {
    pub file: String,
    pub line: u32,
    pub col: u32,
}

impl std::fmt::Display for PanicLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PanicLocation { file, line, col } = self;

        write!(f, "{file}:{line}:{col}")
    }
}

impl From<chillpill::PanicLocation> for PanicLocation {
    fn from(value: chillpill::PanicLocation) -> Self {
        Self {
            file: value.file,
            line: value.line,
            col: value.col,
        }
    }
}
