# CRCNT Macros

## Sample
```rust
use crcnt_ddd_macros::Domain;

#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(table_name = "t_rice", params_extractor = "super::mysql_tools::params_inspect")]
struct __Rice__ {
    id:          String,
    name:        String,
    #[domain_value(skip_new_type = true)]
    create_time: CreateAt,
    update_time: UpdateAt,
    #[domain_value(skip_new_type = true)]
    creator:     Creator,
    updater:     Updater,
    deleted:     Deleted,
}
```
