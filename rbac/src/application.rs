use {crate::{service::Service,
             store::Store},
     chrono::Duration,
     crcnt_ddd_macros::Domain,
     mysql_async::Pool};

#[derive(Clone, Domain)]
#[domain_commands(builder)]
pub struct Config {
  pool:                      Pool,
  session_expiration:        Duration,
  password_salt:             String,
  password_max_failed_times: u8,
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
    let service = Service { session_expiration:        config.session_expiration,
                            password_salt:             config.password_salt,
                            password_max_failed_times: config.password_max_failed_times, };
    Self { store, service }
  }
}

mod feature_admin;
mod operator_admin;
mod role_admin;
mod session_admin;

pub use {feature_admin::ApplicationFeatureAdmin,
         operator_admin::ApplicationOperatorAdmin,
         role_admin::ApplicationRoleAdmin,
         session_admin::ApplicationSessionAdmin};
