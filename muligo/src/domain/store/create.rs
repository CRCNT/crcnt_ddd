use {crate::domain::{error::{MulingoError,
                             Result},
                     store::Store,
                     MulingoEntity,
                     MulingoEntityCRUDExec},
     async_trait::async_trait};

#[async_trait]
pub trait StoreCreate {
  async fn insert_mulingo_entity(&self, mulingo: &MulingoEntity) -> Result<()>;
}

#[async_trait]
impl StoreCreate for Store {
  async fn insert_mulingo_entity(&self, mulingo: &MulingoEntity) -> Result<()> {
    let mut conn = self.pool.get_conn().await.map_err(|e| MulingoError::DatabaseError(e.to_string()))?;
    self.exec_insert_mulingo_entity(mulingo, &mut conn)
        .await
        .map_err(|e| MulingoError::DatabaseError(e.to_string()))
  }
}
