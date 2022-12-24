use {crate::domain::{application::Application,
                     error::Result,
                     service::{ServiceFactory,
                               ServiceVerify},
                     store::{StoreCreate,
                             StoreQuery},
                     MulingoEntity,
                     MulingoLangKey,
                     MulingoMsgContent,
                     MulingoMsgKey,
                     MulingoNameSpace},
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationCreate {
  async fn create_mulingo(&self,
                          ns: MulingoNameSpace,
                          lang_key: MulingoLangKey,
                          msg_key: MulingoMsgKey,
                          msg: MulingoMsgContent)
                          -> Result<MulingoEntity>;
}

#[async_trait]
impl ApplicationCreate for Application {
  async fn create_mulingo(&self,
                          ns: MulingoNameSpace,
                          lang_key: MulingoLangKey,
                          msg_key: MulingoMsgKey,
                          msg: MulingoMsgContent)
                          -> Result<MulingoEntity> {
    let _ = self.store.check_duplicated(&ns, &msg_key, &lang_key).await?;
    let entity = self.service.create_mulingo_entity(ns, lang_key, msg_key, msg)?;
    let _ = self.service.verify_mulingo_entity(&entity)?;
    let _ = self.store.insert_mulingo_entity(&entity).await?;
    Ok(entity)
  }
}
