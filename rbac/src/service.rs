mod factory;
mod hasher;
mod verify;

#[derive(Clone)]
pub struct Service {
  pub(crate) session_expiration: chrono::Duration,
  pub(crate) password_salt:      String,
}

pub use {factory::ServiceFactory,
         hasher::ServiceHasher,
         verify::ServiceVerify};
