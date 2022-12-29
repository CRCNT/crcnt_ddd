use {crate::{application::Application,
             error::Result,
             operator::{OperatorEntity,
                        OperatorName,
                        OperatorNameType},
             service::{ServiceFactory,
                       ServiceVerify},
             session::SessionId,
             store::{StoreCreate,
                     StoreQuery}},
     async_trait::async_trait,
     tracing::debug};

#[async_trait]
pub trait ApplicationOperatorAdmin {
  async fn create_operator_with_login_name(&self, session_id: SessionId, name: OperatorName) -> Result<OperatorEntity>;
}

#[async_trait]
impl ApplicationOperatorAdmin for Application {
  async fn create_operator_with_login_name(&self, session_id: SessionId, name: OperatorName) -> Result<OperatorEntity> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;

    let _ = self.store.check_operator_duplicated(&name).await?;
    // create the entity
    let entity = self.service.create_operator_entity(&session, name, OperatorNameType::LoginName)?;
    debug!("created new operator: {:?}", entity);

    // insert the entity
    let _ = self.store.insert_operator_entity(&entity).await?;
    Ok(entity)
  }
}
