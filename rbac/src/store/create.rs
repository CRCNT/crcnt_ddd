use {crate::{error::{Error::DatabaseError,
                     Result},
             feature::{FeatureEntity,
                       FeatureEntityCRUDExec},
             operator::{OperatorEntity,
                        OperatorEntityCRUDExec,
                        RoleOperatorEntity,
                        RoleOperatorEntityCRUDExec},
             role::{RoleEntity,
                    RoleEntityCRUDExec,
                    RoleFeatureEntity,
                    RoleFeatureEntityCRUDExec},
             session::{SessionEntity,
                       SessionEntityCRUDExec},
             store::Store},
     async_trait::async_trait,
     mysql_async::TxOpts,
     tracing::debug};

#[async_trait]
pub trait StoreCreate {
  async fn insert_operator_entity(&self, operator: &OperatorEntity) -> Result<()>;
  async fn insert_session_entity(&self, session: &SessionEntity) -> Result<()>;
  async fn insert_feature_entity(&self, feature: &FeatureEntity) -> Result<()>;
  async fn insert_role_entity(&self, role: &RoleEntity) -> Result<()>;
  async fn insert_role_features(&self, role_features: Vec<RoleFeatureEntity>) -> Result<()>;
  async fn insert_role_operators(&self, role_operators: Vec<RoleOperatorEntity>) -> Result<()>;
}

#[async_trait]
impl StoreCreate for Store {
  async fn insert_operator_entity(&self, operator: &OperatorEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    let mut txn = conn.start_transaction(TxOpts::default())
                      .await
                      .map_err(|e| DatabaseError(e.to_string()))?;

    debug!("before insert: {:?}", operator);
    self.exec_insert_operator_entity(&operator, &mut txn)
        .await
        .map_err(|e| DatabaseError(e.to_string()))?;
    debug!("after insert: {:?}", operator);

    let _ = txn.commit().await.map_err(|e| DatabaseError(e.to_string()))?;

    Ok(())
  }

  async fn insert_session_entity(&self, session: &SessionEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_insert_session_entity(session, &mut conn)
        .await
        .map_err(|e| DatabaseError(e.to_string()))
  }

  async fn insert_feature_entity(&self, feature: &FeatureEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_insert_feature_entity(feature, &mut conn)
        .await
        .map_err(|e| DatabaseError(e.to_string()))
  }

  async fn insert_role_entity(&self, role: &RoleEntity) -> Result<()> {
    let mut conn = self.get_conn().await?;
    self.exec_insert_role_entity(role, &mut conn)
        .await
        .map_err(|e| DatabaseError(e.to_string()))
  }

  async fn insert_role_features(&self, role_features: Vec<RoleFeatureEntity>) -> Result<()> {
    let mut conn = self.get_conn().await?;
    let mut txn = conn.start_transaction(TxOpts::default())
                      .await
                      .map_err(|e| DatabaseError(e.to_string()))?;

    for role_feature in role_features {
      let _ = self.exec_insert_role_feature_entity(&role_feature, &mut txn)
                  .await
                  .map_err(|e| DatabaseError(e.to_string()))?;
    }
    let _ = txn.commit().await.map_err(|e| DatabaseError(e.to_string()))?;
    Ok(())
  }

  async fn insert_role_operators(&self, role_operators: Vec<RoleOperatorEntity>) -> Result<()> {
    let mut conn = self.get_conn().await?;
    let mut txn = conn.start_transaction(TxOpts::default())
                      .await
                      .map_err(|e| DatabaseError(e.to_string()))?;

    for role_operator in role_operators {
      let _ = self.exec_insert_role_operator_entity(&role_operator, &mut txn)
                  .await
                  .map_err(|e| DatabaseError(e.to_string()))?;
    }
    let _ = txn.commit().await.map_err(|e| DatabaseError(e.to_string()))?;
    Ok(())
  }
}
