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
     crcnt_ddd::value::{Creator,
                        Owner},
     tracing::debug};

#[async_trait]
pub trait ApplicationOperatorAdmin {
  async fn create_operator_with_login_name(&self, session_id: SessionId, owner: Owner, name: OperatorName) -> Result<OperatorEntity>;
}

#[async_trait]
impl ApplicationOperatorAdmin for Application {
  async fn create_operator_with_login_name(&self, session_id: SessionId, owner: Owner, name: OperatorName) -> Result<OperatorEntity> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    // create the entity
    let creator: Creator = session.ref_operator_id().inner().inner().into();
    let entity = self.service.create_operator_entity(owner, creator, name, OperatorNameType::LoginName)?;
    debug!("created new operator: {:?}", entity);
    // verify the entity
    let _ = self.service.verify_operator_entity(&entity)?;
    // insert the entity
    let _ = self.store.insert_operator_entity(&entity).await?;
    Ok(entity)
  }
}
