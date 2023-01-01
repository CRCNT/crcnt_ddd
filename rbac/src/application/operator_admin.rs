use {crate::{application::Application,
             operator::{OperatorEntity,
                        OperatorName,
                        OperatorNameType,
                        OperatorPassword},
             service::{ServiceFactory,
                       ServiceVerify},
             session::SessionId,
             store::{StoreCreate,
                     StoreDelete,
                     StoreQuery,
                     StoreUpdate}},
     anyhow::Result,
     async_trait::async_trait,
     crcnt_ddd::value::Owner,
     tracing::debug};

#[async_trait]
pub trait ApplicationOperatorAdmin {
  async fn create_admin_operator(&self, owner: Owner, name: OperatorName, password: Option<OperatorPassword>) -> Result<OperatorEntity>;
  async fn create_operator_with_login_name(&self, session_id: &SessionId, name: OperatorName) -> Result<OperatorEntity>;
  async fn change_password(&self, session_id: &SessionId, old_password: OperatorPassword, new_password: OperatorPassword) -> Result<()>;
}

#[async_trait]
impl ApplicationOperatorAdmin for Application {
  async fn create_admin_operator(&self, owner: Owner, name: OperatorName, password: Option<OperatorPassword>) -> Result<OperatorEntity> {
    let _ = self.store.check_operator_duplicated(&name).await?;
    // create the entity
    let entity = self.service.create_admin_operator_entity(owner, name, password)?;
    debug!("created new operator: {:?}", entity);

    // insert the entity
    let _ = self.store.insert_operator_entity(&entity).await?;
    Ok(entity)
  }

  async fn create_operator_with_login_name(&self, session_id: &SessionId, name: OperatorName) -> Result<OperatorEntity> {
    // check the session
    let session = self.store.get_session(session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;

    let _ = self.store.check_operator_duplicated(&name).await?;
    // create the entity
    let entity = self.service.create_operator_entity(&session, name, OperatorNameType::LoginName)?;
    debug!("created new operator: {:?}", entity);

    // insert the entity
    let _ = self.store.insert_operator_entity(&entity).await?;
    Ok(entity)
  }

  async fn change_password(&self, session_id: &SessionId, old_password: OperatorPassword, new_password: OperatorPassword) -> Result<()> {
    // check the session
    let session = self.store.get_session(session_id).await?;
    let _ = self.service.verify_session_availability_ignore_type(&session)?;

    // check the old password is correct
    let operator = self.store.get_operator(session.ref_operator_id()).await?;
    let _ = self.service.verify_operator_password(&operator, &old_password)?;

    // check the new password
    let _ = self.service.verify_updating_password(&old_password, &new_password)?;

    // update password
    let operator = self.service.update_operator_password(operator, new_password)?;
    let _ = self.store.update_operator_entity(&operator).await?;

    // delete the session
    let _ = self.store.delete_session_by_id(session.ref_id()).await?;
    Ok(())
  }
}
