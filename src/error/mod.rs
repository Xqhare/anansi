use std::fmt;

pub type AnansiResult<T> = Result<T, AnansiError>;

#[derive(Debug)]
pub enum AnansiError {
    Generic(String),
    Io(std::io::Error),
    InvalidID(String),
    MissingCompletionDate(usize),
}

impl fmt::Display for AnansiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnansiError::Generic(msg) => write!(f, "{}", msg),
            AnansiError::Io(err) => write!(f, "{}", err),
            AnansiError::InvalidID(msg) => write!(f, "{}", msg),
            AnansiError::MissingCompletionDate(id) => {
                write!(
                    f,
                    "Missing completion date for task with ID {}. If a Task has a inception date set, the standard requires a completion date to be set as well.",
                    id
                )
            }
        }
    }
}

impl std::error::Error for AnansiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnansiError::Generic(_)
            | AnansiError::InvalidID(_)
            | AnansiError::MissingCompletionDate(_) => None,
            AnansiError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for AnansiError {
    fn from(err: std::io::Error) -> Self {
        AnansiError::Io(err)
    }
}
