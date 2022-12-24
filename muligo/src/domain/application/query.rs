use {crate::domain::{application::Application,
                     error::Result,
                     store::StoreQuery,
                     MulingoEntity,
                     MulingoLangKey,
                     MulingoMsgKey,
                     MulingoNameSpace,
                     MulingoOwner,
                     MulingoVersion},
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationQuery {
  async fn fetch_latest_mulingo(&self,
                                owner: &MulingoOwner,
                                ns: &MulingoNameSpace,
                                msg_key: &MulingoMsgKey,
                                lang_key: &MulingoLangKey)
                                -> Result<Option<MulingoEntity>>;
  async fn fetch_mulingo(&self,
                         owner: &MulingoOwner,
                         ns: &MulingoNameSpace,
                         msg_key: &MulingoMsgKey,
                         lang_key: &MulingoLangKey,
                         version: &MulingoVersion)
                         -> Result<Option<MulingoEntity>>;
}

#[async_trait]
impl ApplicationQuery for Application {
  async fn fetch_latest_mulingo(&self,
                                owner: &MulingoOwner,
                                ns: &MulingoNameSpace,
                                msg_key: &MulingoMsgKey,
                                lang_key: &MulingoLangKey)
                                -> Result<Option<MulingoEntity>> {
    let mut all_version = self.store.get_mulingo_of_all_version(owner, ns, msg_key, lang_key).await?;
    all_version.sort_by(|x1, x2| x2.version.inner().partial_cmp(x1.version.inner()).unwrap());
    Ok(all_version.first().map(|x| x.clone()))
  }

  async fn fetch_mulingo(&self,
                         owner: &MulingoOwner,
                         ns: &MulingoNameSpace,
                         msg_key: &MulingoMsgKey,
                         lang_key: &MulingoLangKey,
                         version: &MulingoVersion)
                         -> Result<Option<MulingoEntity>> {
    self.store.fetch_mulingo(owner, ns, version, msg_key, lang_key)
  }
}
