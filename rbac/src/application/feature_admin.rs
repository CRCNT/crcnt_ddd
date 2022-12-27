use {crate::{application::Application,
             error::Result,
             feature::{FeatureCode,
                       FeatureDescription,
                       FeatureEndpoint,
                       FeatureEntity,
                       FeatureId,
                       FeatureName,
                       FeatureParentId},
             service::{ServiceFactory,
                       ServiceVerify},
             session::SessionId,
             store::{StoreCreate,
                     StoreQuery}},
     async_trait::async_trait,
     crcnt_ddd::value::Owner};

#[async_trait]
pub trait ApplicationFeatureAdmin {
  async fn create_feature_entity(&self,
                                 session_id: SessionId,
                                 owner: Owner,
                                 parent_id: Option<FeatureParentId>,
                                 code: FeatureCode,
                                 name: FeatureName,
                                 endpoint: Option<FeatureEndpoint>,
                                 description: Option<FeatureDescription>)
                                 -> Result<FeatureEntity>;
  async fn create_top_feature_entity(&self,
                                     session_id: SessionId,
                                     owner: Owner,
                                     code: FeatureCode,
                                     name: FeatureName,
                                     endpoint: Option<FeatureEndpoint>,
                                     description: Option<FeatureDescription>)
                                     -> Result<FeatureEntity> {
    self.create_feature_entity(session_id, owner, None, code, name, endpoint, description)
        .await
  }
}

#[async_trait]
impl ApplicationFeatureAdmin for Application {
  async fn create_feature_entity(&self,
                                 session_id: SessionId,
                                 owner: Owner,
                                 parent_id: Option<FeatureParentId>,
                                 code: FeatureCode,
                                 name: FeatureName,
                                 endpoint: Option<FeatureEndpoint>,
                                 description: Option<FeatureDescription>)
                                 -> Result<FeatureEntity> {
    // check the session
    let session = self.store.get_session(&session_id).await?;
    let _ = self.service.verify_session_availability(&session)?;

    // check if parent_id exists
    if let Some(ref parent_id) = parent_id {
      let parent_feature_id: FeatureId = parent_id.inner().into();
      let _ = self.store.get_feature(&parent_feature_id).await?;
    }

    // check code duplicates
    let _ = self.store.check_feature_code_duplicated(&code).await?;

    // create entity
    let creator = session.as_creator();
    let feature = self.service
                      .create_feature_entity(owner, creator, parent_id, code, name, endpoint, description)?;

    // insert entity
    let _ = self.store.insert_feature_entity(&feature).await?;

    Ok(feature)
  }
}
