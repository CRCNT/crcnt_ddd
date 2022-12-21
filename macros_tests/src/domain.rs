use {crcnt_ddd::value::{CreateAt,
                        Creator,
                        Deleted,
                        UpdateAt,
                        Updater},
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
  #[domain_values(skip)]
  update_time: UpdateAt,
  #[domain_values(skip)]
  creator:     Creator,
  #[domain_values(skip)]
  updater:     Updater,
  #[domain_values(skip)]
  deleted:     Option<Deleted>,
}
