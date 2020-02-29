#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Sync + Send>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    EmptyBody(String),
    TemplateExists(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|c| &**c as &(dyn std::error::Error + 'static))
    }
}

impl Error {
    pub fn new(kind: ErrorKind, source: Option<Box<dyn std::error::Error + Sync + Send>>) -> Self {
        Error { kind, source }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            ErrorKind::EmptyBody(ref message) => write!(f, "{}", message),
            ErrorKind::TemplateExists(ref message) => write!(f, "{}", message),
        }
    }
}
