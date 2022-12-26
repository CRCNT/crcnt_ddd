use {crcnt_rbac::includes::{RBACApplication,
                            RBACConfig},
     mysql_async::Pool,
     tracing_subscriber::{fmt,
                          layer::SubscriberExt,
                          util::SubscriberInitExt,
                          EnvFilter}};

pub fn init() -> RBACApplication {
  tracing_subscriber::registry().with(fmt::Layer::default())
                                .with(EnvFilter::new("warn,crcnt_rbac=debug,crcnt_mulingo=debug,test_admin=debug"))
                                .try_init()
                                .unwrap();
  let pool: Pool = Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo?pool_min=5&pool_max=30");
  let config = RBACConfig::builder().pool(pool)
                                    .session_expiration(chrono::Duration::minutes(30))
                                    .password_salt("A!B".into())
                                    .build();
  let app = RBACApplication::new(config);
  app
}
