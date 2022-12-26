use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialOrd, PartialEq, ThisError)]
pub enum Error {
  #[error("Database error: {0}")]
  DatabaseError(String),

  #[error("Session Not Found")]
  SessionNotFound,

  #[error("Operator Not Found")]
  OperatorNotFound,

  #[error("Password Mismatched")]
  PasswordMismatch,
}
