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
  let owner: MulingoOwner = "SYS_PROMOTION".into();
  let lang_key: MulingoLangKey = "en".into();
  let version: MulingoVersion = "0.1.0".into();
  let version: MulingoVersion = "0.1.1".into();
  let version: MulingoVersion = "0.1.2".into();
  let msg_key: MulingoMsgKey = "EC_600001".into();
  let msg: MulingoMsgContent = "The promotion code is overflow".into();
  let memo: Option<MulingoMemo> = Some("error code for overflow".into());

  let entity = app.create_mulingo(owner.clone(),
                                  ns.clone(),
                                  lang_key.clone(),
                                  msg_key.clone(),
                                  version.clone(),
                                  msg.clone(),
                                  memo.clone())
                  .await;
  info!("created entity: {:?}", entity);
  let msg_key: MulingoMsgKey = "EC_600002".into();
  let entity = app.fetch_latest_mulingo(&owner, &ns, &msg_key, &lang_key).await?;
  info!("fetched: {:?}", entity);

  Ok(())
}
