use {crate::{error::{Error::{self,
                             DatabaseError},
                     Result},
             includes::SessionId,
             operator::OperatorId,
             session::SessionEntityCRUDExec,
             store::Store},
     async_trait::async_trait,
     crcnt_ddd::value::Owner,
     mysql_common::params};

#[async_trait]
pub trait StoreDelete {
  async fn delete_session(&self, owner: &Owner, operator_id: &OperatorId) -> Result<()>;
  async fn delete_session_by_id(&self, session_id: &SessionId) -> Result<()>;
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

  async fn delete_session_by_id(&self, session_id: &SessionId) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_delete_by_id_session_entity(session_id, &mut conn)
        .await
        .map_err(|e| DatabaseError(e.to_string()))
  }
}
