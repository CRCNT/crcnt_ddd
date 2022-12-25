use {crcnt_ddd::value::{CreateAt,
                        Deleted,
                        EntityId,
                        Owner,
                        StrIr,
                        UpdateAt},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value}};

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(table_name = "t_rbac_feature", params_extractor = "crate::params_extractor")]
struct __Feature__ {
  id:          EntityId,
  #[domain_value(optional = true)]
  parent_id:   String,
  code:        String,
  name:        String,
  #[domain_value(optional = true)]
  endpoint:    String,
  #[domain_value(optional = true)]
  description: String,
  #[domain_value(enums = "Active|Inactive")]
  status:      String,
  #[domain_value(skip_new_type = true)]
  owner:       Owner,
  #[domain_value(skip_new_type = true)]
  create_at:   CreateAt,
  #[domain_value(skip_new_type = true)]
  update_at:   UpdateAt,
  #[domain_value(skip_new_type = true)]
  deleted:     Deleted,
}

impl FromValue for FeatureId {
  type Intermediate = StrIr;
}
impl ConvIr<FeatureId> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> FeatureId {
    let creator = String::from_utf8_lossy(&self.bytes).to_string();
    FeatureId(EntityId::new(creator))
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
