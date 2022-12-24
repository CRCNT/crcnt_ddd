use {crcnt_mulingo::includes::*,
     mysql_async::Pool,
     tracing::info,
     tracing_subscriber::{fmt,
                          layer::SubscriberExt,
                          util::SubscriberInitExt,
                          EnvFilter}};

#[tokio::test]
async fn test() -> anyhow::Result<()> {
  tracing_subscriber::registry().with(fmt::Layer::default())
                                .with(EnvFilter::new("warn,crcnt_mulingo=debug,mulingo=debug"))
                                .try_init()
                                .unwrap();

  let pool: Pool = Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo");

  let app = Application::new(pool);

  let ns: MulingoNameSpace = "com.payby.promotion".into();
  let lang_key: MulingoLangKey = "en".into();
  let msg_key: MulingoMsgKey = "EC_600001".into();
  let msg: MulingoMsgContent = "The promotion code is overflow".into();

  let entity = app.create_mulingo(ns, lang_key, msg_key, msg).await?;
  info!("created entity: {:?}", entity);

  Ok(())
}
