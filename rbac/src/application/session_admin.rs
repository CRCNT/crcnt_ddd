use {crate::{application::Application,
             feature::FeatureEntity,
             operator::{OperatorName,
                        OperatorPassword},
             service::{ServiceFactory,
                       ServiceVerify},
             session::{SessionEntity,
                       SessionId},
             store::{StoreCreate,
                     StoreDelete,
                     StoreQuery,
                     StoreUpdate}},
     anyhow::Result,
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationSessionAdmin {
  async fn login_with_name_password(&self, name: OperatorName, password: OperatorPassword) -> Result<SessionEntity>;
  async fn hit_session(&self, session_id: &SessionId) -> Result<SessionEntity>;
  async fn fetch_session_features(&self, session_id: &SessionId) -> Result<Vec<FeatureEntity>>;
}

#[async_trait]
impl ApplicationSessionAdmin for Application {
  async fn login_with_name_password(&self, name: OperatorName, password: OperatorPassword) -> Result<SessionEntity> {
    let operator = self.store.get_operator_by_name(&name).await?;
    let _ = self.service.verify_operator_availability(&operator)?;
    // verify password
    let verify_result = self.service.verify_operator_password(&operator, &password);
    if verify_result.is_err() {
      let operator = self.service.increase_operator_failed_times(operator.clone());
      let _ = self.store.update_operator_entity(&operator).await?;
    }
    let _ = verify_result?;
    // delete all existed session
    let owner = operator.ref_owner();
    let _ = self.store.delete_session(owner, operator.ref_id()).await?;
    // create new session
    let session = self.service.create_session_entity(&operator)?;
    // insert new session
    let _ = self.store.insert_session_entity(&session).await?;
    Ok(session)
  }

  async fn hit_session(&self, session_id: &SessionId) -> Result<SessionEntity> {
    let session = self.store.get_session(session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;
    let session = self.service.hit_session_entity(session)?;
    let _ = self.store.update_session_entity(&session).await?;
    Ok(session)
  }

  async fn fetch_session_features(&self, session_id: &SessionId) -> Result<Vec<FeatureEntity>> {
    // find user's role
    let session = self.store.get_session(session_id).await?;
    let _ = self.service.verify_normal_session_availability(&session)?;

    let operator_id = session.ref_operator_id();
    let role_ids = self.store.get_operator_role_ids(operator_id).await?;
    let feature_ids = self.store.get_feature_ids(role_ids).await?;
    let features = self.store.get_features(feature_ids).await?;

    Ok(features)
  }
}
