use {crate::{error::{Error,
                     Result},
             operator::{OperatorEntity,
                        OperatorEntityCRUDExec,
                        OperatorName},
             session::{SessionEntity,
                       SessionEntityCRUDExec,
                       SessionId},
             store::Store},
     async_trait::async_trait,
     crcnt_ddd::value::Owner,
     mysql_common::params};

#[async_trait]
pub trait StoreQuery {
  async fn get_session(&self, session_id: &SessionId) -> Result<SessionEntity>;
  async fn find_operator(&self, owner: &Owner, name: &OperatorName) -> Result<Option<OperatorEntity>>;
  async fn get_operator_by_name(&self, owner: &Owner, name: &OperatorName) -> Result<OperatorEntity> {
    let xs = self.find_operator(owner, name).await?;
    if let Some(x) = xs { Ok(x) } else { Err(Error::OperatorNotFound) }
  }
}

#[async_trait]
impl StoreQuery for Store {
  async fn get_session(&self, session_id: &SessionId) -> Result<SessionEntity> {
    let mut conn = self.get_conn().await?;
    let session: Option<SessionEntity> = self.exec_get_session_entity(session_id, &mut conn)
                                             .await
                                             .map_err(|e| Error::DatabaseError(e.to_string()))?;

    if let Some(session) = session {
      Ok(session)
    } else {
      Err(Error::SessionNotFound)
    }
  }

  async fn find_operator(&self, owner: &Owner, name: &OperatorName) -> Result<Option<OperatorEntity>> {
    let mut conn = self.get_conn().await?;
    let xs: Vec<OperatorEntity> = self.exec_select_where_operator_entity("WHERE owner = :owner AND name = :name",
                                                                         params! {
                                                                           "owner" => owner.inner(),
                                                                           "name" => name.inner()
                                                                         },
                                                                         &mut conn)
                                      .await
                                      .map_err(|e| Error::DatabaseError(e.to_string()))?;
    Ok(xs.first().map(|x| x.clone()))
  }
}
