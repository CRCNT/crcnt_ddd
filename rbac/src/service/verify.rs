use crate::{error::Result,
            includes::OperatorPassword,
            operator::OperatorEntity,
            service::{Service,
                      ServiceHasher}};

pub trait ServiceVerify {
  fn verify_operator_entity(&self, operator: &OperatorEntity) -> Result<()>;
  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()>;
}

impl ServiceVerify for Service {
  fn verify_operator_entity(&self, _operator: &OperatorEntity) -> Result<()> {
    // FIXME: do the verification logic
    Ok(())
  }

  fn verify_operator_password(&self, operator: &OperatorEntity, password: &OperatorPassword) -> Result<()> {
    self.sha256_verify_password(&self.password_salt, password, operator.ref_password())
  }
}
