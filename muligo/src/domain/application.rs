use {crate::domain::{service::Service,
                     store::Store},
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
mod query;
mod update;

pub use {create::ApplicationCreate,
         query::ApplicationQuery,
         update::ApplicationUpdate};
