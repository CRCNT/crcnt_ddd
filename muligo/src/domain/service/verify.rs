use crate::domain::{error::Result,
                    service::Service,
                    MulingoEntity};

pub trait ServiceVerify {
  fn verify_mulingo_entity(&self, mulingo: &MulingoEntity) -> Result<()>;
}

impl ServiceVerify for Service {
  fn verify_mulingo_entity(&self, _mulingo: &MulingoEntity) -> Result<()> {
    // add length verification
    Ok(())
  }
}
