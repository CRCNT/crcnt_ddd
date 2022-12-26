use {crate::{application::Application,
             error::Result,
             operator::{OperatorName,
                        OperatorPassword},
             service::{ServiceFactory,
                       ServiceVerify},
             session::SessionEntity,
             store::{StoreCreate,
                     StoreDelete,
                     StoreQuery}},
     async_trait::async_trait,
     crcnt_ddd::value::Owner};

#[async_trait]
pub trait ApplicationSessionLogin {
  async fn login_with_name_password(&self, owner: Owner, name: OperatorName, password: OperatorPassword) -> Result<SessionEntity>;
}

#[async_trait]
impl ApplicationSessionLogin for Application {
  async fn login_with_name_password(&self, owner: Owner, name: OperatorName, password: OperatorPassword) -> Result<SessionEntity> {
    let operator = self.store.get_operator_by_name(&owner, &name).await?;
    // verify password
    let _ = self.service.verify_operator_password(&operator, &password)?;
    // delete all existed session
    let _ = self.store.delete_session(&owner, operator.ref_id()).await?;
    // create new session
    let session = self.service.create_session_entity(owner, operator.mv_id())?;
    // insert new session
    let _ = self.store.insert_session_entity(&session).await?;
    Ok(session)
  }
}
