use {crate::feature::FeatureId,
     crcnt_ddd::value::{CreateAt,
                        Creator,
                        Deleted,
                        EntityId,
                        Owner,
                        StrIr,
                        UpdateAt,
                        Updater},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value}};

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(table_name = "t_rbac_role", params_extractor = "crate::params_extractor")]
struct __Role__ {
  id:          EntityId,
  code:        String,
  name:        String,
  #[domain_value(optional = true)]
  description: String,
  level:       u8,
  #[domain_value(enums = "Active|Inactive")]
  status:      String,
  #[domain_value(skip_new_type = true)]
  owner:       Owner,
  #[domain_value(skip_new_type = true)]
  creator:     Creator,
  #[domain_value(skip_new_type = true)]
  updater:     Updater,
  #[domain_value(skip_new_type = true)]
  create_at:   CreateAt,
  #[domain_value(skip_new_type = true)]
  update_at:   UpdateAt,
  #[domain_value(skip_new_type = true)]
  deleted:     Deleted,
}

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(params_extractor = "crate::params_extractor", table_name = "t_rbac_role_feature")]
struct __RoleFeature__ {
  id:         EntityId,
  #[domain_value(skip_new_type = true)]
  role_id:    RoleId,
  #[domain_value(skip_new_type = true)]
  feature_id: FeatureId,
  #[domain_value(skip_new_type = true)]
  owner:      Owner,
  #[domain_value(skip_new_type = true)]
  create_at:  CreateAt,
}

impl FromValue for RoleId {
  type Intermediate = StrIr;
}
impl ConvIr<RoleId> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> RoleId {
    let creator = String::from_utf8_lossy(&self.bytes).to_string();
    RoleId(EntityId::new(creator))
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
