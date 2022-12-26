use {crate::{error::{Error,
                     Result},
             operator::OperatorId,
             session::SessionEntityCRUDExec,
             store::Store},
     async_trait::async_trait,
     crcnt_ddd::value::Owner,
     mysql_common::params};

#[async_trait]
pub trait StoreDelete {
  async fn delete_session(&self, owner: &Owner, operator_id: &OperatorId) -> Result<()>;
}

#[async_trait]
impl StoreDelete for Store {
  async fn delete_session(&self, owner: &Owner, operator_id: &OperatorId) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_delete_where_session_entity("WHERE owner = :owner AND operator_id = :operator_id",
                                          params! {
                                            "owner" => owner.inner(),
                                            "operator_id" => operator_id.inner(),
                                          },
                                          &mut conn)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
  }
}
