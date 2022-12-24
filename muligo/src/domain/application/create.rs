use {crate::domain::{application::Application,
                     error::Result,
                     service::{ServiceFactory,
                               ServiceVerify},
                     store::{StoreCreate,
                             StoreQuery},
                     MulingoEntity,
                     MulingoLangKey,
                     MulingoMemo,
                     MulingoMsgContent,
                     MulingoMsgKey,
                     MulingoNameSpace,
                     MulingoOwner,
                     MulingoVersion},
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationCreate {
  async fn create_mulingo(&self,
                          owner: MulingoOwner,
                          ns: MulingoNameSpace,
                          lang_key: MulingoLangKey,
                          msg_key: MulingoMsgKey,
                          version: MulingoVersion,
                          msg: MulingoMsgContent,
                          memo: Option<MulingoMemo>)
                          -> Result<MulingoEntity>;
}

#[async_trait]
impl ApplicationCreate for Application {
  async fn create_mulingo(&self,
                          owner: MulingoOwner,
                          ns: MulingoNameSpace,
                          lang_key: MulingoLangKey,
                          msg_key: MulingoMsgKey,
                          version: MulingoVersion,
                          msg: MulingoMsgContent,
                          memo: Option<MulingoMemo>)
                          -> Result<MulingoEntity> {
    let _ = self.store.check_duplicated(&owner, &ns, &version, &msg_key, &lang_key).await?;
    let entity = self.service.create_mulingo_entity(ns, owner, lang_key, msg_key, version, msg, memo)?;
    let _ = self.service.verify_mulingo_entity(&entity)?;
    let _ = self.store.insert_mulingo_entity(&entity).await?;
    Ok(entity)
  }
}
