use {crcnt_ddd::value::CreateAt,
     crcnt_ddd_macros::{DomainEntity,
                        DomainStore,
                        DomainValues}};

#[allow(dead_code)]
#[derive(DomainEntity, DomainValues, DomainStore)]
struct __Rice__ {
  id:          String,
  name:        String,
  #[domain_values(skip)]
  create_time: CreateAt,
  deleted:     Option<bool>,
}
