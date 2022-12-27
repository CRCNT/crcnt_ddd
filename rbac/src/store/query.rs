use {crate::{error::{Error::*,
                     Result},
             feature::{FeatureCode,
                       FeatureEntity,
                       FeatureEntityCRUDExec,
                       FeatureId},
             operator::{OperatorEntity,
                        OperatorEntityCRUDExec,
                        OperatorName},
             session::{SessionEntity,
                       SessionEntityCRUDExec,
                       SessionId},
             store::Store},
     async_trait::async_trait,
     crcnt_ddd::value::Owner,
     mysql_common::params};

#[async_trait]
pub trait StoreQuery {
  async fn get_session(&self, session_id: &SessionId) -> Result<SessionEntity>;
  async fn find_operator(&self, owner: &Owner, name: &OperatorName) -> Result<Option<OperatorEntity>>;
  async fn get_feature(&self, feature_id: &FeatureId) -> Result<FeatureEntity>;
  async fn get_feature_by_code(&self, feature_code: &FeatureCode) -> Result<Option<FeatureEntity>>;
  async fn check_feature_code_duplicated(&self, feature_code: &FeatureCode) -> Result<()> {
    let feature = self.get_feature_by_code(feature_code).await?;
    if let Some(_feature) = feature {
      Err(FeatureCodeDuplicated)
    } else {
      Ok(())
    }
  }
  async fn get_operator_by_name(&self, owner: &Owner, name: &OperatorName) -> Result<OperatorEntity> {
    let xs = self.find_operator(owner, name).await?;
    if let Some(x) = xs { Ok(x) } else { Err(OperatorNotFound) }
  }
}

#[async_trait]
impl StoreQuery for Store {
  async fn get_session(&self, session_id: &SessionId) -> Result<SessionEntity> {
    let mut conn = self.get_conn().await?;
    let session: Option<SessionEntity> = self.exec_get_session_entity(session_id, &mut conn)
                                             .await
                                             .map_err(|e| DatabaseError(e.to_string()))?;

    if let Some(session) = session {
      Ok(session)
    } else {
      Err(SessionNotFound)
    }
  }

  async fn find_operator(&self, owner: &Owner, name: &OperatorName) -> Result<Option<OperatorEntity>> {
    let mut conn = self.get_conn().await?;
    let xs: Vec<OperatorEntity> = self.exec_select_where_operator_entity("WHERE owner = :owner AND name = :name",
                                                                         params! {
                                                                           "owner" => owner.inner(),
                                                                           "name" => name.inner()
                                                                         },
                                                                         &mut conn)
                                      .await
                                      .map_err(|e| DatabaseError(e.to_string()))?;
    Ok(xs.first().map(|x| x.clone()))
  }

  async fn get_feature(&self, feature_id: &FeatureId) -> Result<FeatureEntity> {
    let mut conn = self.get_conn().await?;
    let feature: Option<FeatureEntity> = self.exec_get_feature_entity(feature_id, &mut conn)
                                             .await
                                             .map_err(|e| DatabaseError(e.to_string()))?;
    if let Some(feature) = feature {
      Ok(feature)
    } else {
      Err(FeatureNotFound)
    }
  }

  async fn get_feature_by_code(&self, feature_code: &FeatureCode) -> Result<Option<FeatureEntity>> {
    let mut conn = self.get_conn().await?;
    let features: Vec<FeatureEntity> = self.exec_select_where_feature_entity("WHERE code = :code",
                                                                             params! {
                                                                               "code" => feature_code.inner()
                                                                             },
                                                                             &mut conn)
                                           .await
                                           .map_err(|e| DatabaseError(e.to_string()))?;
    Ok(features.first().map(|x| x.clone()))
  }
}
