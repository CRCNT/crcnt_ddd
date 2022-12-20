mod domain;

use {crate::domain::*,
     chrono::NaiveDateTime,
     crcnt_ddd::value::CreateAt,
     mysql_async::{params,
                   prelude::{Query,
                             ToConnection,
                             WithParams},
                   TxOpts},
     std::any::Any};

pub struct SomeStore {}

impl RiceBasicStoreHelper for SomeStore {}

#[tokio::test]
async fn test_values() -> anyhow::Result<()> {
  let rice = RiceEntity::builder().id("01")
                                  .name("东北大米")
                                  .create_time(CreateAt::now())
                                  .deleted(false)
                                  .unsafe_build();
  println!("rice: {:?}", rice);
  let pool = mysql_async::Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo");

  let store = SomeStore {};
  let sql = store.sql_insert_rice();
  let params = store.mysql_params_rice(&rice);
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

  let xs: mysql_async::Result<Vec<(String, String, CreateAt, bool)>> =
    store.exec_select_rice("where id = :id", params! {"id" => "01"}, &mut conn).await;

  println!("xs: {:?}", xs);
  Ok(())
}
