use {crate::domain::{error::Result,
                     service::Service,
                     MulingoEntity,
                     MulingoLangKey,
                     MulingoMsgContent,
                     MulingoMsgKey,
                     MulingoNameSpace},
     crcnt_ddd::value::{CreateAt,
                        UpdateAt},
     ulid::Ulid};

pub trait ServiceFactory {
  fn create_mulingo_entity(&self,
                           ns: MulingoNameSpace,
                           lang_key: MulingoLangKey,
                           msg_key: MulingoMsgKey,
                           msg: MulingoMsgContent)
                           -> Result<MulingoEntity>;
}

impl ServiceFactory for Service {
  fn create_mulingo_entity(&self,
                           ns: MulingoNameSpace,
                           lang_key: MulingoLangKey,
                           msg_key: MulingoMsgKey,
                           msg: MulingoMsgContent)
                           -> Result<MulingoEntity> {
    Ok(MulingoEntity::builder().id(Ulid::new().to_string().into())
                               .name_space(ns)
                               .msg_key(msg_key)
                               .lang_key(lang_key)
                               .msg_content(msg)
                               .create_at(CreateAt::now())
                               .update_at(UpdateAt::now())
                               .build())
  }
}
