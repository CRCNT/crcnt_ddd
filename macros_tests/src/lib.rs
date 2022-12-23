// mod domain_entity;

mod rice {
  use {crcnt_ddd::value::{CreateAt,
                          Creator,
                          Deleted,
                          UpdateAt,
                          Updater},
       crcnt_ddd_macros::Domain};

  #[derive(Domain)]
  #[domain_commands(entity, store)]
  #[domain_store(table_name = "t_rice", params_extractor = "super::mysql_tools::params_inspect")]
  struct __Rice__ {
    id:          String,
    name:        String,
    #[domain_value(skip_new_type = true)]
    create_time: CreateAt,
    update_time: UpdateAt,
    #[domain_value(skip_new_type = true)]
    creator:     Creator,
    updater:     Updater,
    deleted:     Deleted,
  }
}

mod mysql_tools;

mod test {
  use {super::rice::*,
       crcnt_ddd::value::{CreateAt,
                          Creator,
                          Deleted,
                          UpdateAt,
                          Updater},
       mysql_async::Pool,
       mysql_common::params::Params,
       tracing_subscriber::{fmt,
                            prelude::__tracing_subscriber_SubscriberExt,
                            util::SubscriberInitExt,
                            EnvFilter}};

  struct Store;
  impl RiceEntityCRUDStmt for Store {}
  impl RiceEntityCRUDExec for Store {}

  #[tokio::test]
  async fn test_macros() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(fmt::Layer::default())
                                  .with(EnvFilter::new("warn,macros_tests=debug"))
                                  .try_init()
                                  .unwrap();

    let pool: Pool = Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo");
    let mut conn = pool.get_conn().await?;

    let rice = RiceEntity::builder().id("01".into())
                                    .name("山东大米".into())
                                    .create_time(CreateAt::now())
                                    .update_time(UpdateAt::now().into())
                                    .creator(Creator::new("zenas"))
                                    .updater(RiceUpdater::new(Updater::new("zenas")))
                                    .deleted(RiceDeleted::new(Deleted::new(false)))
                                    .build();

    match Store.exec_insert_rice_entity(&rice, &mut conn).await {
      | Ok(_) => {}
      | Err(e) => {
        eprintln!("e: {}", e.to_string());
      }
    }

    let rice_id = RiceId::new("01");
    let rice: Option<RiceEntity> = Store.exec_get_rice_entity(&rice_id, &mut conn).await?;
    println!("{:?}", rice);
    let rice = rice.unwrap();
    let rice = rice.set_name("哥斯达黎加大米".into());
    let _ = Store.exec_update_rice_entity(&rice, &mut conn).await?;
    let rice: Option<RiceEntity> = Store.exec_get_rice_entity(&rice_id, &mut conn).await?;
    println!("{:?}", rice);

    let _ = Store.exec_delete_where_rice_entity("where 1 = 1", Params::Empty, &mut conn).await?;

    Ok(())
  }
}
