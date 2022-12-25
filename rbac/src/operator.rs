use {crate::role::RoleId,
     crcnt_ddd::value::{CreateAt,
                        Deleted,
                        EntityId,
                        Owner,
                        StrIr,
                        UpdateAt,
                        UtcDateTime},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value}};

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(params_extractor = "crate::params_extractor", table_name = "t_rbac_operator")]
struct __Operator__ {
  id:            EntityId,
  profile_id:    String,
  name:          String,
  #[domain_value(enums = "LoginName|Email|Mobile")]
  name_type:     String,
  password:      String,
  #[domain_value(enums = "Active|Inactive")]
  status:        String,
  last_login_at: UtcDateTime,
  failed_times:  u8,
  #[domain_value(skip_new_type = true)]
  owner:         Owner,
  #[domain_value(skip_new_type = true)]
  create_at:     CreateAt,
  #[domain_value(skip_new_type = true)]
  update_at:     UpdateAt,
  #[domain_value(skip_new_type = true)]
  deleted:       Deleted,
}

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(params_extractor = "crate::params_extractor", table_name = "t_rbac_operator_role")]
struct __OperatorRole__ {
  id:          EntityId,
  #[domain_value(skip_new_type = true)]
  operator_id: OperatorId,
  #[domain_value(skip_new_type = true)]
  role_id:     RoleId,
  #[domain_value(skip_new_type = true)]
  owner:       Owner,
  #[domain_value(skip_new_type = true)]
  create_at:   CreateAt,
}

impl FromValue for OperatorId {
  type Intermediate = StrIr;
}
impl ConvIr<OperatorId> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> OperatorId {
    let creator = String::from_utf8_lossy(&self.bytes).to_string();
    OperatorId(EntityId::new(creator))
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}

#[cfg(test)]
mod test {
  use {super::*,
       ulid::Ulid};

  struct Store;

  impl OperatorEntityCRUDStmt for Store {}

  impl OperatorEntityCRUDExec for Store {}
  #[test]
  fn test() {
    let operator = OperatorEntity::builder().id(EntityId::new(Ulid::new()).into())
                                            .profile_id("M01".into())
                                            .name("admin".into())
                                            .name_type(OperatorNameType::LoginName)
                                            .password("12345".into())
                                            .status(OperatorStatus::Active)
                                            .last_login_at(UtcDateTime::now().into())
                                            .failed_times(0u8.into())
                                            .owner("CNT".into())
                                            .create_at(CreateAt::now())
                                            .update_at(UpdateAt::now())
                                            .deleted(true.into())
                                            .build();
    println!("test: {:?}", operator);
    println!("select sql: {}", Store.stmt_select_operator_entity());
  }
}
