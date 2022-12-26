use {crate::session::{SessionEntity,
                      SessionEntityCRUDExec},
     async_trait::async_trait};

use crate::{error::{Error,
                    Result},
            store::Store};

#[async_trait]
pub trait StoreUpdate {
  async fn update_session_entity(&self, session: &SessionEntity) -> Result<()>;
}

#[async_trait]
impl StoreUpdate for Store {
  async fn update_session_entity(&self, session: &SessionEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_update_session_entity(session, &mut conn)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
  }
}
