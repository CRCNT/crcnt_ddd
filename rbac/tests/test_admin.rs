use {anyhow::Result,
     crcnt_ddd::value::Owner,
     crcnt_rbac::includes::{OperatorName,
                            OperatorPassword,
                            OperatorStatus,
                            RBACApplication,
                            RBACApplicationCreate,
                            RBACApplicationSessionLogin,
                            RBACConfig,
                            SessionId},
     mysql_async::Pool,
     tracing::info,
     tracing_subscriber::{fmt,
                          layer::SubscriberExt,
                          util::SubscriberInitExt,
                          EnvFilter}};

#[tokio::test]
async fn test_add_operator() -> Result<()> {
  println!("{}", OperatorStatus::NeedChangePwd.inner());

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

  let owner = Owner::new("SYS");
  let name = OperatorName::new("admin");
  let password = OperatorPassword::new("passw0rd!");

  let session = app.login_with_name_password(owner, name.clone(), password.clone()).await?;

  let session: SessionId = session.ref_id().clone();

  let owner = Owner::new("PROMO");
  let operator = app.create_operator_with_login_name(session, owner, name).await?;

  info!("{:?}", operator);

  Ok(())
}
