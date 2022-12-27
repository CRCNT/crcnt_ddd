use {crate::{error::{Error::SessionExpired,
                     Result},
             includes::OperatorPassword,
             operator::OperatorEntity,
             service::{Service,
                       ServiceHasher},
             session::SessionEntity},
     crcnt_ddd::value::UtcDateTime};

pub trait ServiceVerify {
  fn verify_operator_entity(&self, operator: &OperatorEntity) -> Result<()>;
  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()>;
  fn verify_session_expiration(&self, session: &SessionEntity) -> Result<()>;
}

impl ServiceVerify for Service {
  fn verify_operator_entity(&self, _operator: &OperatorEntity) -> Result<()> {
    // FIXME: do the verification logic
    Ok(())
  }

  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()> {
    self.sha256_verify_password(&self.password_salt, password, operator.ref_password())
  }

  fn verify_session_expiration(&self, session: &SessionEntity) -> Result<()> {
    let expire = session.ref_expire_at().inner();
    let now = UtcDateTime::now();

    if expire < &now {
      // expired
      Err(SessionExpired)
    } else {
      Ok(())
    }
  }
}
