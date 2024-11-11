use core::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    DatabaseError { message: String, code: i32 },
    DataBaseConnectionError { message: String, code: i32 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::DatabaseError { message, code } => {
                write!(f, "Database error {}: {}", code, message)
            }
            Self::DataBaseConnectionError { message, code } => {
                write!(f, "Database connection error {}: {}", code, message)
            }
        }
    }
}

impl std::error::Error for Error {}
