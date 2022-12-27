use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialOrd, PartialEq, ThisError)]
pub enum Error {
  #[error("Database error: {0}")]
  DatabaseError(String),

  #[error("Session Not Found")]
  SessionNotFound,

  #[error("Session Expired")]
  SessionExpired,

  #[error("Operator Not Found")]
  OperatorNotFound,

  #[error("Operator Inactive")]
  OperatorInactive,

  #[error("Operator Using Initial Password")]
  OperatorNeedChangePassword,

  #[error("Operator Login Failed Too Many Times")]
  OperatorTooManyFailedLogin,

  #[error("Password Mismatched")]
  PasswordMismatch,

  #[error("Feature Not Found")]
  FeatureNotFound,

  #[error("Feature Code Duplicated")]
  FeatureCodeDuplicated,
}
