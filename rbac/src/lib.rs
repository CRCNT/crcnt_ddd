use mysql_common::{params::Params,
                   value::Value};

mod access;
mod application;
mod error;
mod feature;
mod operator;
mod role;
mod service;
mod session;
mod store;

#[allow(unused)]
pub(crate) fn params_extractor(_params: &mysql_async::Params) -> String {
  match _params {
    | Params::Empty => "Empty".to_string(),
    | Params::Named(ref named) => {
      let kvs = named.iter()
                     .map(|(k, v)| {
                       let key = String::from_utf8_lossy(k).to_string();
                       let v = match v {
                         | Value::NULL => "NULL".to_string(),
                         | Value::Bytes(ref bytes) => String::from_utf8_lossy(bytes).to_string(),
                         | Value::Int(i) => i.to_string(),
                         | Value::UInt(i) => i.to_string(),
                         | Value::Float(i) => i.to_string(),
                         | Value::Double(i) => i.to_string(),
                         | Value::Date(_, _, _, _, _, _, _) => "Date".to_string(),
                         | Value::Time(_, _, _, _, _, _) => "Time".to_string(),
                       };
                       format!("{} = {}", key, v)
                     })
                     .collect::<Vec<_>>();
      kvs.join(",")
    }
    | Params::Positional(_) => "Position".to_string(),
  }
}

pub mod includes {
  pub use super::{application::{Application as RBACApplication,
                                ApplicationFeatureAdmin as RBACApplicationFeatureAdmin,
                                ApplicationOperatorAdmin as RBACApplicationOperatorAdmin,
                                ApplicationRoleAdmin as RBACApplicationRoleAdmin,
                                ApplicationSessionAdmin as RBACApplicationSessionAdmin,
                                Config as RBACConfig},
                  error::Error,
                  feature::{FeatureCode,
                            FeatureDescription,
                            FeatureEndpoint,
                            FeatureId,
                            FeatureName},
                  operator::{OperatorId,
                             OperatorName,
                             OperatorNameType,
                             OperatorPassword,
                             OperatorStatus},
                  role::{RoleCode,
                         RoleDescription,
                         RoleEntity,
                         RoleId,
                         RoleLevel,
                         RoleName},
                  session::{SessionEntity,
                            SessionId}};
}
