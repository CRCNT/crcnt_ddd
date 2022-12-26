use {crate::{service::Service,
             store::Store},
     chrono::Duration,
     crcnt_ddd_macros::Domain,
     mysql_async::Pool};

#[derive(Clone, Domain)]
#[domain_commands(builder)]
pub struct Config {
  pool:               Pool,
  session_expiration: Duration,
  password_salt:      String,
}

#[derive(Clone)]
pub struct Application {
  pub(crate) store:   Store,
  pub(crate) service: Service,
}

impl Application {
  pub fn new(config: Config) -> Self {
    let pool = config.pool;

    let store = Store { pool };
    let service = Service { session_expiration: config.session_expiration,
                            password_salt:      config.password_salt, };
    Self { store, service }
  }
}

mod operator_create;
mod session_login;

pub use {operator_create::ApplicationCreate,
         session_login::ApplicationSessionLogin};
