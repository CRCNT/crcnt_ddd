use {crate::domain::{MulingoEntityCRUDExec,
                     MulingoEntityCRUDStmt},
     mysql_async::Pool,
     mysql_common::params::Params};

pub(crate) fn params_extractor(_params: &Params) -> String { "No dump".to_string() }

#[derive(Clone)]
pub(crate) struct Store {
  pub(crate) pool: Pool,
}
impl MulingoEntityCRUDStmt for Store {}
impl MulingoEntityCRUDExec for Store {}

mod create;
mod query;

pub use {create::StoreCreate,
         query::StoreQuery};
