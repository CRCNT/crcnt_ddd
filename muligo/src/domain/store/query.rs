use {crate::domain::{error::{MulingoError,
                             Result},
                     store::Store,
                     MulingoEntity,
                     MulingoEntityCRUDExec,
                     MulingoLangKey,
                     MulingoMsgKey,
                     MulingoNameSpace},
     async_trait::async_trait,
     mysql_common::params};

#[async_trait]
pub trait StoreQuery {
  async fn query_message_content(&self, ns: &MulingoNameSpace, msg_key: &MulingoMsgKey, lang_key: &MulingoLangKey) -> Result<Option<MulingoEntity>>;
  async fn check_duplicated(&self, ns: &MulingoNameSpace, msg_key: &MulingoMsgKey, lang_key: &MulingoLangKey) -> Result<()> {
    let xs = self.query_message_content(ns, msg_key, lang_key).await?;
    if xs.is_some() {
      return Err(MulingoError::DuplicatedMsgKey { ns:       ns.inner().clone(),
                                                  lang_key: lang_key.inner().clone(),
                                                  msg_key:  msg_key.inner().clone(), });
    }
    Ok(())
  }
  async fn get_message_content(&self, ns: &MulingoNameSpace, msg_key: &MulingoMsgKey, lang_key: &MulingoLangKey) -> Result<MulingoEntity> {
    let xs = self.query_message_content(ns, msg_key, lang_key).await?;
    if let Some(x) = xs {
      Ok(x)
    } else {
      return Err(MulingoError::NonExistedMsgKey { ns:       ns.inner().clone(),
                                                  lang_key: lang_key.inner().clone(),
                                                  msg_key:  msg_key.inner().clone(), });
    }
  }
}

#[async_trait]
impl StoreQuery for Store {
  async fn query_message_content(&self, ns: &MulingoNameSpace, msg_key: &MulingoMsgKey, lang_key: &MulingoLangKey) -> Result<Option<MulingoEntity>> {
    let mut conn = self.pool.get_conn().await.map_err(|e| MulingoError::DatabaseError(e.to_string()))?;
    let entities: Vec<MulingoEntity> = self.exec_select_where_mulingo_entity("WHERE name_space = :name_space AND msg_key = :msg_key AND lang_key = \
                                                                              :lang_key",
                                                                             params! {
                                                                               "name_space" => ns.inner(),
                                                                               "msg_key" => msg_key.inner(),
                                                                               "lang_key" => lang_key.inner(),
                                                                             },
                                                                             &mut conn)
                                           .await
                                           .map_err(|e| MulingoError::DatabaseError(e.to_string()))?;

    if entities.len() > 1 {
      // duplicated
      return Err(MulingoError::DuplicatedMsgKey { ns:       ns.inner().clone(),
                                                  lang_key: lang_key.inner().clone(),
                                                  msg_key:  msg_key.inner().clone(), });
    }
    Ok(entities.first().map(|x| x.clone()))
  }
}
