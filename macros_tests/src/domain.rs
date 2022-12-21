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
#[domain_store(table_name = "t_rice_v1")]
struct __Rice__ {
  id:          String,
  name:        String,
  #[domain_values(skip = true)]
  create_time: CreateAt,
  #[domain_values(skip = true)]
  update_time: UpdateAt,
  #[domain_values(skip = true)]
  creator:     Creator,
  #[domain_values(skip = true)]
  updater:     Updater,
  #[domain_values(skip = true)]
  deleted:     Option<Deleted>,
  #[domain_values(skip = false, value_kind = "enum", enums = "Active|Inactive")]
  status:      String,
}
