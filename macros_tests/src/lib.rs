mod domain;

use {crate::domain::*,
     crcnt_ddd::value::{CreateAt,
                        Deleted,
                        UpdateAt},
     mysql_async::{params,
                   TxOpts}};

pub struct SomeStore {}

#[test]
fn test_entity() -> anyhow::Result<()> {
  let rice = RiceEntity::builder().id("1")
                                  .name("zenas")
                                  .create_time(CreateAt::now())
                                  .update_time(UpdateAt::now())
                                  .deleted(Some(true))
                                  .unsafe_build();

  println!("{:?}", rice);
  Ok(())
}

impl RiceBasicStoreHelper for SomeStore {}

#[tokio::test]
async fn test_values() -> anyhow::Result<()> {
  let rice = RiceEntity::builder().id("01")
                                  .name("东北大米")
                                  .create_time(CreateAt::now())
                                  .update_time(UpdateAt::now())
                                  .creator("test")
                                  .updater("test")
                                  .deleted(Some(Deleted::new(false)))
                                  .unsafe_build();
  println!("rice: {:?}", rice);
  let pool = mysql_async::Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo");

  let store = SomeStore {};
  let sql = store.sql_insert_rice();
  println!("sql: {}", sql);

  let mut conn = pool.get_conn().await?;
  let mut txn = conn.start_transaction(TxOpts::default()).await?;

  let _ = store.exec_insert_rice(&rice, &mut txn).await?;

  let rice = rice.set_name("日本大米".into());
  let _ = store.exec_update_rice(&rice, &mut txn).await?;
  let rows = txn.affected_rows();
  println!("updated {} rows", rows);

  let _ = txn.commit().await?;
  println!("committed");

  let xs: mysql_async::Result<Vec<RiceEntity>> = store.exec_select_rice("where id = :id", params! {"id" => "01"}, &mut conn).await;

  println!("xs: {:?}", xs);

  let id: RiceId = RiceId::new("01");
  let rice_entity: Option<RiceEntity> = store.exec_get_rice(&id, &mut conn).await?;
  dbg!(rice_entity);
  Ok(())
}
