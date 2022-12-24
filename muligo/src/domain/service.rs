#[derive(Clone)]
pub struct Service;

mod factory;
mod verify;

pub use {factory::ServiceFactory,
         verify::ServiceVerify};
