/// Inspect the static error codes from an error enum.
pub trait InspectErrorCode {
  fn error_code(&self) -> &str;
}
