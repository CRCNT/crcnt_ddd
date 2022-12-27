use {anyhow::Result,
     crcnt_ddd::value::Owner,
     crcnt_rbac::includes::{OperatorName,
                            OperatorPassword,
                            RBACApplication,
                            RBACApplicationSessionAdmin,
                            RBACConfig,
                            SessionEntity},
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
                                    .password_max_failed_times(5)
                                    .build();
  let app = RBACApplication::new(config);
  app
}

pub async fn login(app: &RBACApplication) -> Result<SessionEntity> {
  let owner = Owner::new("SYS");
  let name = OperatorName::new("ROOT");
  let password = OperatorPassword::new("passw0rd!");

  let session = app.login_with_name_password(owner, name.clone(), password.clone()).await?;
  Ok(session)
}
