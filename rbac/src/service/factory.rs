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
                        OperatorName,
                        OperatorNameType,
                        OperatorPassword,
                        OperatorStatus,
                        RoleOperatorEntity},
             role::{RoleCode,
                    RoleDescription,
                    RoleEntity,
                    RoleFeatureEntity,
                    RoleLevel,
                    RoleName,
                    RoleStatus},
             service::{Service,
                       ServiceHasher},
             session::{SessionEntity,
                       SessionExpireAt,
                       SessionLastHitAt,
                       SessionLoginAt,
                       SessionSessionType}},
     crcnt_ddd::value::{CreateAt,
                        Creator,
                        EntityId,
                        Owner,
                        UpdateAt,
                        UtcDateTime}};

pub trait ServiceFactory {
  fn create_operator_entity(&self, session: &SessionEntity, name: OperatorName, name_type: OperatorNameType) -> Result<OperatorEntity>;
  fn create_session_entity(&self, operator: &OperatorEntity) -> Result<SessionEntity>;
  fn create_feature_entity(&self,
                           creator: Creator,
                           parent_id: Option<FeatureParentId>,
                           code: FeatureCode,
                           name: FeatureName,
                           endpoint: Option<FeatureEndpoint>,
                           description: Option<FeatureDescription>)
                           -> Result<FeatureEntity>;
  fn create_role_entity(&self,
                        session: &SessionEntity,
                        code: RoleCode,
                        name: RoleName,
                        level: RoleLevel,
                        description: Option<RoleDescription>)
                        -> Result<RoleEntity>;
  fn create_role_features(&self, session: &SessionEntity, role: RoleEntity, features: Vec<FeatureEntity>) -> Result<Vec<RoleFeatureEntity>>;
  fn create_role_operators(&self, session: &SessionEntity, role: RoleEntity, operators: Vec<OperatorEntity>) -> Result<Vec<RoleOperatorEntity>>;
  fn hit_session_entity(&self, session: SessionEntity) -> Result<SessionEntity>;
  fn increase_operator_failed_times(&self, operator: OperatorEntity) -> OperatorEntity {
    let failed_times: OperatorFailedTimes = OperatorFailedTimes::new(*(operator.ref_failed_times().inner()) + 1);
    operator.set_failed_times(failed_times).set_update_at(UpdateAt::now())
  }
}

impl ServiceFactory for Service {
  fn create_operator_entity(&self, session: &SessionEntity, name: OperatorName, name_type: OperatorNameType) -> Result<OperatorEntity> {
    let owner: Owner = session.as_owner();
    let creator: Creator = session.as_creator();
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

  fn create_session_entity(&self, operator: &OperatorEntity) -> Result<SessionEntity> {
    let login_at = SessionLoginAt::new(UtcDateTime::now());
    let owner = operator.ref_owner().clone();
    let expire_at: SessionExpireAt = (login_at.inner().clone() + self.session_expiration.clone()).into();
    let session_type = if operator.ref_status() == &OperatorStatus::NeedChangePwd {
      SessionSessionType::ChangePassword
    } else {
      SessionSessionType::Normal
    };
    Ok(SessionEntity::builder().id(EntityId::new_with_prefix("SS").into())
                               .operator_id(operator.ref_id().clone())
                               .data(None)
                               .session_type(session_type)
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

  fn create_role_entity(&self,
                        session: &SessionEntity,
                        code: RoleCode,
                        name: RoleName,
                        level: RoleLevel,
                        description: Option<RoleDescription>)
                        -> Result<RoleEntity> {
    let owner: Owner = session.as_owner();
    let creator: Creator = session.as_creator();
    Ok(RoleEntity::builder().id(EntityId::new_with_prefix("RL").into())
                            .code(code)
                            .name(name)
                            .description(description)
                            .level(level)
                            .status(RoleStatus::Active)
                            .creator(creator.inner().into())
                            .updater(creator.inner().into())
                            .owner(owner)
                            .create_at(CreateAt::now())
                            .update_at(UpdateAt::now())
                            .deleted(false.into())
                            .build())
  }

  fn create_role_features(&self, session: &SessionEntity, role: RoleEntity, features: Vec<FeatureEntity>) -> Result<Vec<RoleFeatureEntity>> {
    let owner = session.ref_owner();
    Ok(features.iter()
               .map(|feature| {
                 RoleFeatureEntity::builder().id(EntityId::new_with_prefix("RF").into())
                                             .role_id(role.ref_id().clone())
                                             .feature_id(feature.ref_id().clone())
                                             .owner(owner.clone())
                                             .create_at(CreateAt::now())
                                             .build()
               })
               .collect::<Vec<_>>())
  }

  fn create_role_operators(&self, session: &SessionEntity, role: RoleEntity, operators: Vec<OperatorEntity>) -> Result<Vec<RoleOperatorEntity>> {
    let owner = session.ref_owner();
    Ok(operators.iter()
                .map(|operator| {
                  RoleOperatorEntity::builder().id(EntityId::new_with_prefix("RO").into())
                                               .role_id(role.ref_id().clone())
                                               .operator_id(operator.ref_id().clone())
                                               .owner(owner.clone())
                                               .create_at(CreateAt::now())
                                               .build()
                })
                .collect::<Vec<_>>())
  }

  fn hit_session_entity(&self, session: SessionEntity) -> Result<SessionEntity> {
    let now = UtcDateTime::now();
    let expire = now.clone() + self.session_expiration.clone();
    let session = session.set_last_hit_at(SessionLastHitAt::new(now.clone())).set_expire_at(expire.into());
    Ok(session)
  }
}
