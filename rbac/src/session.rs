use {crate::operator::OperatorId,
     crcnt_ddd::value::{Creator,
                        EntityId,
                        Owner,
                        StrIr,
                        UtcDateTime},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value}};

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(table_name = "t_rbac_session", params_extractor = "crate::params_extractor")]
struct __Session__ {
  id:          EntityId,
  #[domain_value(skip_new_type = true)]
  operator_id: OperatorId,
  #[domain_value(optional = true)]
  data:        String,
  login_at:    UtcDateTime,
  last_hit_at: UtcDateTime,
  expire_at:   UtcDateTime,
  #[domain_value(skip_new_type = true)]
  owner:       Owner,
}

impl FromValue for SessionId {
  type Intermediate = StrIr;
}
impl ConvIr<SessionId> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> SessionId {
    let creator = String::from_utf8_lossy(&self.bytes).to_string();
    SessionId(EntityId::new(creator))
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}

impl SessionEntity {
  pub fn as_creator(&self) -> Creator { self.ref_operator_id().inner().inner().into() }
}
