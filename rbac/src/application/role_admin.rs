use {crate::{application::Application,
             error::Result,
             feature::FeatureId,
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
     async_trait::async_trait,
     crcnt_ddd::value::Owner};

#[async_trait]
pub trait ApplicationRoleAdmin {
  async fn create_role(&self,
                       session_id: SessionId,
                       owner: Owner,
                       code: RoleCode,
                       name: RoleName,
                       level: RoleLevel,
                       description: Option<RoleDescription>)
                       -> Result<RoleEntity>;

  async fn set_role_features(&self, session_id: SessionId, role_id: RoleId, features: Vec<FeatureId>) -> Result<()>;
}

#[async_trait]
impl ApplicationRoleAdmin for Application {
  async fn create_role(&self,
                       session_id: SessionId,
                       owner: Owner,
                       code: RoleCode,
                       name: RoleName,
                       level: RoleLevel,
                       description: Option<RoleDescription>)
                       -> Result<RoleEntity> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_session_availability(&session)?;
    // create the entity
    let creator = session.as_creator();
    let entity = self.service.create_role_entity(owner, creator, code, name, level, description)?;
    // insert the entity
    let _ = self.store.insert_role_entity(&entity).await?;
    Ok(entity)
  }

  async fn set_role_features(&self, session_id: SessionId, role_id: RoleId, features: Vec<FeatureId>) -> Result<()> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_session_availability(&session)?;

    // check the role id and feature ids.
    let role = self.store.get_role(&role_id).await?;
    let features = self.store.get_features(features).await?;
    let role_features = self.service.create_role_features(&session, role, features)?;
    let _ = self.store.insert_role_features(role_features).await?;
    Ok(())
  }
}
