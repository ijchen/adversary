#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PanicInfo {
    pub message: Option<String>,
    pub location: Option<PanicLocation>,
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

impl From<&std::panic::Location<'_>> for PanicLocation {
    fn from(value: &std::panic::Location<'_>) -> Self {
        Self {
            file: value.file().to_string(),
            line: value.line(),
            col: value.column(),
        }
    }
}
