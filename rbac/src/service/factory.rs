use {crate::{error::Result,
             feature::{FeatureCode,
                       FeatureDescription,
                       FeatureEndpoint,
                       FeatureEntity,
                       FeatureName,
                       FeatureParentId,
                       FeatureStatus},
             operator::{OperatorEntity,
                        OperatorFailedTimes,
                        OperatorId,
                        OperatorName,
                        OperatorNameType,
                        OperatorPassword,
                        OperatorStatus},
             service::{Service,
                       ServiceHasher},
             session::{SessionEntity,
                       SessionExpireAt,
                       SessionLastHitAt,
                       SessionLoginAt}},
     crcnt_ddd::value::{CreateAt,
                        Creator,
                        EntityId,
                        Owner,
                        UpdateAt,
                        UtcDateTime}};

pub trait ServiceFactory {
  fn create_operator_entity(&self, owner: Owner, creator: Creator, name: OperatorName, name_type: OperatorNameType) -> Result<OperatorEntity>;
  fn create_session_entity(&self, owner: Owner, operator_id: OperatorId) -> Result<SessionEntity>;
  fn create_feature_entity(&self,
                           creator: Creator,
                           parent_id: Option<FeatureParentId>,
                           code: FeatureCode,
                           name: FeatureName,
                           endpoint: Option<FeatureEndpoint>,
                           description: Option<FeatureDescription>)
                           -> Result<FeatureEntity>;
  fn hit_session_entity(&self, session: SessionEntity) -> Result<SessionEntity>;
  fn increase_operator_failed_times(&self, operator: OperatorEntity) -> OperatorEntity {
    let failed_times: OperatorFailedTimes = OperatorFailedTimes::new(*(operator.ref_failed_times().inner()) + 1);
    operator.set_failed_times(failed_times).set_update_at(UpdateAt::now())
  }
}

impl ServiceFactory for Service {
  fn create_operator_entity(&self, owner: Owner, creator: Creator, name: OperatorName, name_type: OperatorNameType) -> Result<OperatorEntity> {
    let password = OperatorPassword::change_me();
    let password = self.sha256_hash_password(&self.password_salt, password)?;
    Ok(OperatorEntity::builder().owner(owner)
                                .name(name)
                                .name_type(name_type)
                                .password(password)
                                .status(OperatorStatus::NeedChangePwd)
                                .create_at(CreateAt::now())
                                .update_at(UpdateAt::now())
                                .deleted(false.into())
                                .id(EntityId::new_with_prefix("OP").into())
                                .profile_id(None)
                                .last_login_at(None)
                                .creator(creator.inner().into())
                                .updater(creator.inner().into())
                                .failed_times(0u8.into())
                                .build())
  }

  fn create_session_entity(&self, owner: Owner, operator_id: OperatorId) -> Result<SessionEntity> {
    let login_at = SessionLoginAt::new(UtcDateTime::now());
    let expire_at: SessionExpireAt = (login_at.inner().clone() + self.session_expiration.clone()).into();
    Ok(SessionEntity::builder().id(EntityId::new_with_prefix("SS").into())
                               .operator_id(operator_id)
                               .data(None)
                               .login_at(login_at)
                               .last_hit_at(UtcDateTime::now().into())
                               .expire_at(expire_at)
                               .owner(owner)
                               .build())
  }

  fn create_feature_entity(&self,
                           creator: Creator,
                           parent_id: Option<FeatureParentId>,
                           code: FeatureCode,
                           name: FeatureName,
                           endpoint: Option<FeatureEndpoint>,
                           description: Option<FeatureDescription>)
                           -> Result<FeatureEntity> {
    Ok(FeatureEntity::builder().id(EntityId::new_with_prefix("FT").into())
                               .parent_id(parent_id)
                               .code(code)
                               .name(name)
                               .endpoint(endpoint)
                               .description(description)
                               .status(FeatureStatus::Active)
                               .creator(creator.inner().into())
                               .updater(creator.inner().into())
                               .create_at(CreateAt::now())
                               .update_at(UpdateAt::now())
                               .deleted(false.into())
                               .build())
  }

  fn hit_session_entity(&self, session: SessionEntity) -> Result<SessionEntity> {
    let now = UtcDateTime::now();
    let expire = now.clone() + self.session_expiration.clone();
    let session = session.set_last_hit_at(SessionLastHitAt::new(now.clone())).set_expire_at(expire.into());
    Ok(session)
  }
}
