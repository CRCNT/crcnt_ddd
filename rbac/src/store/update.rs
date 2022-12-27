use {crate::session::{SessionEntity,
                      SessionEntityCRUDExec},
     async_trait::async_trait};

use crate::{error::{Error,
                    Result},
            operator::{OperatorEntity,
                       OperatorEntityCRUDExec},
            store::Store};

#[async_trait]
pub trait StoreUpdate {
  async fn update_session_entity(&self, session: &SessionEntity) -> Result<()>;
  async fn update_operator_entity(&self, operator: &OperatorEntity) -> Result<()>;
}

#[async_trait]
impl StoreUpdate for Store {
  async fn update_session_entity(&self, session: &SessionEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_update_session_entity(session, &mut conn)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
  }

  async fn update_operator_entity(&self, operator: &OperatorEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_update_operator_entity(operator, &mut conn)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
  }
}
