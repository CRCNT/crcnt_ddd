use {crate::{error::{Error::{FeatureAccessNotAuthorized,
                             NewPasswordSameWithOldPassword,
                             OperatorDeleted,
                             OperatorInactive,
                             OperatorNeedChangePassword,
                             OperatorTooManyFailedLogin,
                             PasswordIllegal,
                             PasswordTooWeak,
                             SessionExpired},
                     Result},
             feature::{FeatureCode,
                       FeatureEntity},
             includes::{OperatorPassword,
                        OperatorStatus},
             operator::OperatorEntity,
             service::{Service,
                       ServiceHasher},
             session::{SessionEntity,
                       SessionSessionType}},
     crcnt_ddd::value::UtcDateTime,
     tracing::error};

pub trait ServiceVerify {
  fn verify_operator_availability(&self, operator: &OperatorEntity) -> Result<()>;
  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()>;
  fn verify_normal_session_availability(&self, session: &SessionEntity) -> Result<()>;
  fn verify_session_availability_ignore_type(&self, session: &SessionEntity) -> Result<()>;
  fn verify_updating_password(&self, old_password: &OperatorPassword, new_password: &OperatorPassword) -> Result<()>;
  fn can_access_feature(&self, features: &Vec<FeatureEntity>, feature_code: &FeatureCode) -> Result<()>;
}

impl ServiceVerify for Service {
  fn verify_operator_availability(&self, operator: &OperatorEntity) -> Result<()> {
    if *(operator.ref_deleted().inner()) {
      return Err(OperatorDeleted);
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

  fn verify_normal_session_availability(&self, session: &SessionEntity) -> Result<()> {
    if session.ref_session_type() == &SessionSessionType::ChangePassword {
      return Err(OperatorNeedChangePassword);
    }

    let expire = session.ref_expire_at().inner();
    let now = UtcDateTime::now();

    if expire < &now {
      // expired
      return Err(SessionExpired);
    }
    Ok(())
  }

  fn verify_session_availability_ignore_type(&self, session: &SessionEntity) -> Result<()> {
    let expire = session.ref_expire_at().inner();
    let now = UtcDateTime::now();

    if expire < &now {
      // expired
      return Err(SessionExpired);
    }
    Ok(())
  }

  fn verify_updating_password(&self, old_password: &OperatorPassword, new_password: &OperatorPassword) -> Result<()> {
    if new_password.inner().eq(old_password.inner()) {
      return Err(NewPasswordSameWithOldPassword);
    }
    // check the new password strength
    let entropy = zxcvbn::zxcvbn(new_password.inner(), &[]).map_err(|e| {
                                                             error!("password strength checking error: {}", e.to_string());
                                                             PasswordIllegal
                                                           })?;
    if entropy.score() < 3 {
      return Err(PasswordTooWeak);
    }

    Ok(())
  }

  fn can_access_feature(&self, features: &Vec<FeatureEntity>, feature_code: &FeatureCode) -> Result<()> {
    if let Some(_) = features.iter().find(|&x| x.ref_code().inner().eq(feature_code.inner())) {
      Ok(())
    } else {
      Err(FeatureAccessNotAuthorized)
    }
  }
}
