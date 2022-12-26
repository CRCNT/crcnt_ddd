use {crate::{error::{Error,
                     Result},
             operator::OperatorPassword,
             service::Service},
     sha2::{Digest,
            Sha256},
     tracing::debug};

pub trait ServiceHasher {
  fn sha256_hash_password(&self, salt: &String, password: OperatorPassword) -> Result<OperatorPassword>;
  fn sha256_verify_password(&self, salt: &String, password1: &OperatorPassword, password2: &OperatorPassword) -> Result<()>;
}

trait HasherSource {
  fn source(&self, salt: &String, password: &OperatorPassword) -> String;
}

impl ServiceHasher for Service {
  fn sha256_hash_password(&self, salt: &String, password: OperatorPassword) -> Result<OperatorPassword> {
    let mut hasher = Sha256::new();
    let source = self.source(salt, &password);
    hasher.update(source.as_str());
    let result = hasher.finalize();
    let result = result.as_slice();
    let result = hex::encode(result);
    let password = OperatorPassword::new(result);
    Ok(password)
  }

  fn sha256_verify_password(&self, salt: &String, password1: &OperatorPassword, password2: &OperatorPassword) -> Result<()> {
    let mut hasher = Sha256::new();
    let source = self.source(salt, password1);

    hasher.update(source.as_str());
    let result = hasher.finalize();
    let result = result.as_slice();
    let result = hex::encode(result);
    debug!("sha256_verify_password: password1: {}, password2: {}", result, password2.inner());
    if result.eq(password2.inner()) {
      Ok(())
    } else {
      Err(Error::PasswordMismatch)
    }
  }
}

impl HasherSource for Service {
  #[inline]
  fn source(&self, salt: &String, password: &OperatorPassword) -> String { format!("{}{}", salt, password.inner().as_str()) }
}
