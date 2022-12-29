use {crate::{application::Application,
             error::Result,
             feature::FeatureId,
             includes::OperatorId,
             role::{RoleCode,
                    RoleDescription,
                    RoleEntity,
                    RoleId,
                    RoleLevel,
                    RoleName},
             service::{ServiceFactory,
                       ServiceVerify},
             session::SessionId,
             store::{StoreCreate,
                     StoreQuery}},
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationRoleAdmin {
  async fn create_role(&self,
                       session_id: SessionId,
                       code: RoleCode,
                       name: RoleName,
                       level: RoleLevel,
                       description: Option<RoleDescription>)
                       -> Result<RoleEntity>;

  async fn set_role_features(&self, session_id: SessionId, role_id: RoleId, feature_ids: Vec<FeatureId>) -> Result<()>;
  async fn set_role_operators(&self, session_id: SessionId, role_id: RoleId, operator_ids: Vec<OperatorId>) -> Result<()>;
}

#[async_trait]
impl ApplicationRoleAdmin for Application {
  async fn create_role(&self,
                       session_id: SessionId,
                       code: RoleCode,
                       name: RoleName,
                       level: RoleLevel,
                       description: Option<RoleDescription>)
                       -> Result<RoleEntity> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;
    // create the entity
    let entity = self.service.create_role_entity(&session, code, name, level, description)?;
    // insert the entity
    let _ = self.store.insert_role_entity(&entity).await?;
    Ok(entity)
  }

  async fn set_role_features(&self, session_id: SessionId, role_id: RoleId, feature_ids: Vec<FeatureId>) -> Result<()> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;

    // check the role id and feature ids.
    let role = self.store.get_role(&role_id).await?;
    let features = self.store.get_features(feature_ids).await?;
    let role_features = self.service.create_role_features(&session, role, features)?;
    let _ = self.store.insert_role_features(role_features).await?;
    Ok(())
  }

  async fn set_role_operators(&self, session_id: SessionId, role_id: RoleId, operator_ids: Vec<OperatorId>) -> Result<()> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;

    // check the role id and operator ids.
    let role = self.store.get_role(&role_id).await?;
    let operators = self.store.get_operators(operator_ids).await?;
    let role_operators = self.service.create_role_operators(&session, role, operators)?;
    let _ = self.store.insert_role_operators(role_operators).await?;
    Ok(())
  }
}
