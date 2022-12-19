use {crcnt_ddd_macros::DomainModel,
     serde::{Deserialize,
             Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAt(pub u64);

#[allow(dead_code)]
#[derive(DomainModel, Debug, Clone)]
struct __CustomerInfo__ {
  id:          String,
  name:        String,
  description: Option<String>,
  create_at:   CreateAt,
  update_at:   u64,
}
