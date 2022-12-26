use {crate::{error::Result,
             operator::{OperatorEntity,
                        OperatorId,
                        OperatorName,
                        OperatorNameType,
                        OperatorPassword,
                        OperatorStatus},
             service::{Service,
                       ServiceHasher},
             session::{SessionEntity,
                       SessionExpireAt,
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
}
