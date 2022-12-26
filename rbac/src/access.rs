use {crate::{feature::FeatureCode,
             operator::OperatorId,
             session::SessionId},
     crcnt_ddd::value::{EntityId,
                        Owner,
                        UtcDateTime},
     crcnt_ddd_macros::Domain};

#[doc(hidden)]
#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(table_name = "t_rbac_access", params_extractor = "crate::params_extractor")]
struct __Access__ {
  id:           EntityId,
  #[domain_value(skip_new_type = true)]
  operator_id:  OperatorId,
  #[domain_value(skip_new_type = true)]
  session_id:   SessionId,
  #[domain_value(skip_new_type = true)]
  feature_code: FeatureCode,
  #[domain_value(enums = "Allowed|Forbidden")]
  control:      String,
  access_at:    UtcDateTime,
  #[domain_value(skip_new_type = true)]
  owner:        Owner,
}
