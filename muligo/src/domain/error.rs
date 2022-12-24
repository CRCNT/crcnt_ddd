use thiserror::Error;

pub type Result<T> = std::result::Result<T, MulingoError>;

#[derive(Debug, Clone, PartialOrd, PartialEq, Error)]
pub enum MulingoError {
  #[error("Duplicated \"{msg_key}\" of \"{lang_key}\" in \"{ns}\"")]
  DuplicatedMsgKey { ns: String, lang_key: String, msg_key: String },
  #[error("Non-existed \"{msg_key}\" of \"{lang_key}\" in \"{ns}\"")]
  NonExistedMsgKey { ns: String, lang_key: String, msg_key: String },

  #[error("Database error: {0}")]
  DatabaseError(String),
}
