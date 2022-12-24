use {crcnt_ddd::value::{CreateAt,
                        UpdateAt},
     crcnt_ddd_macros::Domain};

#[derive(Domain)]
#[domain_commands(entity, store)]
#[domain_store(params_extractor = "store::params_extractor")]
struct __Mulingo__ {
  id:          String,
  name_space:  String,
  msg_key:     String,
  lang_key:    String,
  msg_content: String,
  #[domain_value(skip_new_type = true)]
  create_at:   CreateAt,
  #[domain_value(skip_new_type = true)]
  update_at:   UpdateAt,
}

mod application;
mod error;
mod service;
mod store;

pub mod includes {
  pub use super::{application::{Application,
                                ApplicationCreate},
                  MulingoEntity,
                  MulingoLangKey,
                  MulingoMsgContent,
                  MulingoMsgKey,
                  MulingoNameSpace};
}
