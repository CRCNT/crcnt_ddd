use {crate::domain::store::Store,
     mysql_async::Pool};

#[derive(Clone)]
pub struct Application {
  store:   Store,
  service: Service,
}

impl Application {
  pub fn new(pool: Pool) -> Self {
    let store = Store { pool };
    let service = Service;
    Self { store, service }
  }
}

mod create;

use crate::domain::service::Service;
pub use create::ApplicationCreate;
