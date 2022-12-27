use {crate::{error::{Error::{OperatorDeleted,
                             OperatorInactive,
                             OperatorNeedChangePassword,
                             OperatorTooManyFailedLogin,
                             SessionExpired},
                     Result},
             includes::{OperatorPassword,
                        OperatorStatus},
             operator::OperatorEntity,
             service::{Service,
                       ServiceHasher},
             session::SessionEntity},
     crcnt_ddd::value::UtcDateTime};

pub trait ServiceVerify {
  fn verify_operator_entity(&self, operator: &OperatorEntity) -> Result<()>;
  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()>;
  fn verify_session_availability(&self, session: &SessionEntity) -> Result<()>;
}

impl ServiceVerify for Service {
  fn verify_operator_entity(&self, operator: &OperatorEntity) -> Result<()> {
    if *(operator.ref_deleted().inner()) {
      return Err(OperatorDeleted);
    }
    if &OperatorStatus::NeedChangePwd == operator.ref_status() {
      return Err(OperatorNeedChangePassword);
    }
    if &OperatorStatus::Inactive == operator.ref_status() {
      return Err(OperatorInactive);
    }
    if operator.ref_failed_times().inner() > &self.password_max_failed_times {
      return Err(OperatorTooManyFailedLogin);
    }
    Ok(())
  }

  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()> {
    self.sha256_verify_password(&self.password_salt, password, operator.ref_password())
  }

  fn verify_session_availability(&self, session: &SessionEntity) -> Result<()> {
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
